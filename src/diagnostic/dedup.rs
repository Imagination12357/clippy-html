//! Canonical diagnostic equality and deterministic fingerprints.
use super::model::Finding;
use std::collections::BTreeMap;
fn canonical_bytes(f: &Finding) -> Vec<u8> {
    let mut key = f.clone();
    key.id.clear();
    key.targets.clear();
    key.rendered = None;
    serde_json::to_vec(&key).expect("Normalized findings are JSON serializable")
}
fn merge_with(input: Vec<Finding>, digest: impl Fn(&[u8]) -> String) -> Vec<Finding> {
    let mut groups: BTreeMap<Vec<u8>, Finding> = BTreeMap::new();
    for mut f in input {
        for t in &mut f.targets {
            t.kinds.sort();
            t.kinds.dedup();
        }
        let key = canonical_bytes(&f);
        if let Some(existing) = groups.get_mut(&key) {
            existing.targets.extend(f.targets);
        } else {
            groups.insert(key, f);
        }
    }
    let mut collisions: BTreeMap<String, usize> = BTreeMap::new();
    let mut result = vec![];
    for (key, mut f) in groups {
        let hash = digest(&key);
        let index = collisions.entry(hash.clone()).or_default();
        f.id = if *index == 0 {
            hash
        } else {
            format!("{hash}-{index}")
        };
        *index += 1;
        f.targets.sort();
        f.targets.dedup();
        result.push(f);
    }
    result.sort_by(|a, b| {
        a.package_id
            .cmp(&b.package_id)
            .then_with(|| {
                a.spans
                    .first()
                    .map(|s| (&s.path, s.line_start, s.column_start))
                    .cmp(
                        &b.spans
                            .first()
                            .map(|s| (&s.path, s.line_start, s.column_start)),
                    )
            })
            .then(a.code.cmp(&b.code))
            .then(a.level.cmp(&b.level))
            .then(a.id.cmp(&b.id))
    });
    result
}
pub fn merge_findings(input: Vec<Finding>) -> Vec<Finding> {
    merge_with(input, |b| blake3::hash(b).to_hex().to_string())
}
#[cfg(test)]
mod tests {
    use super::*;
    fn sample() -> Finding {
        let cargo_metadata::Message::CompilerMessage(m) =
            serde_json::from_value(super::super::sample()).unwrap()
        else {
            panic!()
        };
        super::super::normalize::normalize(&m, std::path::Path::new("/demo"))
    }
    #[test]
    fn forced_hash_collision_keeps_both() {
        let a = sample();
        let mut b = a.clone();
        b.message = "other".into();
        let r = merge_with(vec![a.clone(), b.clone()], |_| "same".into());
        assert_ne!(r[0].id, r[1].id);
        assert_eq!(r, merge_with(vec![b, a], |_| "same".into()));
    }
    #[test]
    fn merges_targets_only() {
        let a = sample();
        let mut b = a.clone();
        b.targets[0].name = "demo-test".into();
        let r = merge_findings(vec![a, b]);
        assert_eq!(r.len(), 1);
        assert_eq!(r[0].targets.len(), 2);
    }
    #[test]
    fn keeps_package_level_and_suggestion_differences() {
        let a = sample();
        let mut b = a.clone();
        b.package_id = "another".into();
        let mut c = a.clone();
        c.level = "error".into();
        let mut d = a.clone();
        d.children.push(super::super::model::Note {
            level: "help".into(),
            message: "different fix".into(),
            spans: vec![],
            children: vec![],
        });
        assert_eq!(merge_findings(vec![a, b, c, d]).len(), 4);
    }
    #[test]
    fn ordering_and_ids_ignore_message_arrival_order() {
        let a = sample();
        let mut b = a.clone();
        b.message = "other".into();
        let r = merge_findings(vec![a.clone(), b.clone()]);
        let s = merge_findings(vec![b, a]);
        assert_eq!(r, s);
        assert!(!r[0].id.is_empty());
    }
}
