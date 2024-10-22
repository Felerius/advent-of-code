@default:
    just --list

run day year='':
    #!/usr/bin/env bash
    year="$(just _year_or_derive "{{year}}" "{{invocation_directory()}}")"
    cargo run -p "aoc${year}" -- "{{day}}"

run-rel day year='':
    #!/usr/bin/env bash
    year="$(just _year_or_derive "{{year}}" "{{invocation_directory()}}")"
    cargo run -p "aoc${year}" --release -- "{{day}}"

bench day year='':
    #!/usr/bin/env bash
    year="$(just _year_or_derive "{{year}}" "{{invocation_directory()}}")"
    cargo bench -p "aoc${year}" --bench bench -- "day$(printf "%02d" "{{day}}")" --min-time 1 --max-time 5

_year_or_derive year dir:
    #!/usr/bin/env bash
    if [[ -n "{{year}}" ]]; then
        echo "{{year}}"
    else
        dir="{{dir}}"
        while [[ "$dir" != "/" ]] && [[ "$(dirname "$dir")" != "{{justfile_directory()}}" ]]; do
            dir="$(dirname "$dir")"
        done
        if [[ "$(dirname "$dir")" == "{{justfile_directory()}}" ]] && [[ "$(basename "$dir")" =~ ^[0-9]{4}$ ]]; then
            echo "$(basename "$dir")"
        else
            echo "Could not derive year from directory" >&2
            exit 1
        fi
    fi
