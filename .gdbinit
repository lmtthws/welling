python
print "-- Loading Rust pretty-printers --"
import os
sys.path.insert(0,os.path.expandvars("$RUSTUP_HOME/toolchains/stable-x86_64-pc-windows-gnu/lib/rustlib/etc"))
import gdb_rust_pretty_printing
gdb_rust_pretty_printing.register_printers(gdb)
print "-- Pretty-printers loaded --"
end