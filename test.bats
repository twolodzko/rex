#!/usr/bin/env bats

example='
PING www.example.com (93.184.216.34): 56 data bytes
64 bytes from 93.184.216.34: icmp_seq=0 ttl=56 time=11.632 ms
64 bytes from 93.184.216.34: icmp_seq=1 ttl=56 time=11.726 ms
64 bytes from 93.184.216.34: icmp_seq=2 ttl=56 time=10.683 ms
64 bytes from 93.184.216.34: icmp_seq=3 ttl=56 time=9.674 ms
64 bytes from 93.184.216.34: icmp_seq=4 ttl=56 time=11.127 ms

--- www.example.com ping statistics ---
5 packets transmitted, 5 packets received, 0.0% packet loss
round-trip min/avg/max/stddev = 9.674/10.968/11.726/0.748 ms
'

regex='(?P<size>[0-9]+) bytes from (?P<ip>[0-9]+\.[0-9]+\.[0-9]+\.[0-9]+).*time=(?P<time>[0-9\.]+) ms'

@test "Fail with no arguments" {
    run ! ./rex
    [ "$status" -ne 0 ]
}

@test "Fail with empty regex" {
    run ! ./rex ''
    [ "$status" -ne 0 ]
}

@test "Fail with invalid regex" {
    run ! ./rex '['
    [ "$status" -ne 0 ]
}

@test "Fail when file not exists" {
    run ! ./rex '.*' there-is-no-such-file.txt
    [ "$status" -ne 0 ]
}

@test "Extract columns to csv" {
    run ./rex -s ',' "$regex" <<< "$example"
    [ "$status" -eq 0 ]
    [ "${#lines[@]}" -eq 5 ]
    [ "${lines[0]}" = "64,93.184.216.34,11.632" ]
}

@test "Extract columns with line numbers" {
    run ./rex -l -s '###' "$regex" <<< "$example"
    [ "$status" -eq 0 ]
    [ "${#lines[@]}" -eq 5 ]
    [ "${lines[0]}" = "3###64###93.184.216.34###11.632" ]
}

@test "Extract columns to json" {
    run ./rex -j "$regex" <<< "$example"
    [ "$status" -eq 0 ]
    [ "${#lines[@]}" -eq 5 ]
    [ "${lines[0]}" = '{"ip":"93.184.216.34","size":"64","time":"11.632"}' ]
}

@test "Extract columns to json with line numbers" {
    run ./rex -jl "$regex" <<< "$example"
    [ "$status" -eq 0 ]
    [ "${#lines[@]}" -eq 5 ]
    [ "${lines[0]}" = '{"ip":"93.184.216.34","line":"3","size":"64","time":"11.632"}' ]
}
