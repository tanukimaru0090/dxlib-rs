arg = ARGV[0]
res = `cargo test test_dxlib  --features utils    -- #{arg}  --nocapture`
p   res
