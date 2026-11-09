#!/usr/bin/env bash
# Exercise the CI helper's exit status and retained artifacts without compiling.
set -euo pipefail

helper="$PWD/.github/scripts/dogfood.sh"
scratch=$(mktemp -d)
trap 'rm -rf -- "$scratch"' EXIT
mkdir -p "$scratch/bin"
cat > "$scratch/bin/cargo" <<'CARGO'
#!/usr/bin/env bash
set -eu
case "$1" in
  --version|clippy) echo 'test cargo'; exit 0 ;;
  install) echo 'install log'; exit "${INSTALL_STATUS:-0}" ;;
  clippy-html)
    shift
    [ "$1" = --output ]
    mkdir -p "$2"
    printf 'test report\n' > "$2/index.html"
    printf '%s\n' "$*" > "$2/arguments.txt"
    echo 'report log'
    exit "${REPORT_STATUS:-0}"
    ;;
esac
exit 99
CARGO
chmod +x "$scratch/bin/cargo"
export PATH="$scratch/bin:$PATH"

run_helper() {
  local expected=$1 work=$2 actual=0
  bash "$helper" "$work" > "$scratch/output.log" 2>&1 || actual=$?
  if [ "$actual" -ne "$expected" ]; then
    cat "$scratch/output.log"
    echo "expected exit $expected, got $actual" >&2
    exit 1
  fi
}

run_helper 0 "$scratch/success"
test -f "$scratch/success/report/index.html"
grep -q 'install log' "$scratch/success/logs/install.log"
grep -q 'report log' "$scratch/success/logs/report.log"
grep -q -- '-- --locked --all-targets -- -W clippy::pedantic' "$scratch/success/report/arguments.txt"
if grep -q -- '--offline' "$scratch/success/report/arguments.txt"; then exit 1; fi

export INSTALL_STATUS=17
run_helper 17 "$scratch/install-failure"
grep -q 'install log' "$scratch/install-failure/logs/install.log"
test ! -e "$scratch/install-failure/report"
unset INSTALL_STATUS

export REPORT_STATUS=23
run_helper 23 "$scratch/report-failure"
test -f "$scratch/report-failure/report/index.html"
grep -q 'report log' "$scratch/report-failure/logs/report.log"
unset REPORT_STATUS

run_helper 1 "$scratch/success"
grep -q 'test report' "$scratch/success/report/index.html"
echo 'CI dogfood behavior: 4 cases passed'
