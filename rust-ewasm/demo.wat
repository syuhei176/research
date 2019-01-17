(module
  (type $t0 (func (param i32)))
  (type $t1 (func (param i32 i32)))
  (type $t2 (func))
  (import "env" "ethereum_getAddress" (func $ethereum_getAddress (type $t0)))
  (import "env" "ethereum_storageStore" (func $ethereum_storageStore (type $t1)))
  (import "env" "ethereum_storageLoad" (func $ethereum_storageLoad (type $t1)))
  (func $main (export "main") (type $t2)
    (local $l0 i32)
    (global.set $g0
      (local.tee $l0
        (i32.sub
          (global.get $g0)
          (i32.const 96))))
    (i64.store
      (i32.add
        (local.get $l0)
        (i32.const 24))
      (i64.const 0))
    (i64.store
      (i32.add
        (local.get $l0)
        (i32.const 16))
      (i64.const 0))
    (i64.store
      (i32.add
        (local.get $l0)
        (i32.const 8))
      (i64.const 0))
    (i64.store
      (local.get $l0)
      (i64.const 0))
    (i64.store
      (i32.add
        (i32.add
          (local.get $l0)
          (i32.const 32))
        (i32.const 24))
      (i64.const 0))
    (i64.store
      (i32.add
        (i32.add
          (local.get $l0)
          (i32.const 32))
        (i32.const 16))
      (i64.const 0))
    (i64.store
      (i32.add
        (i32.add
          (local.get $l0)
          (i32.const 32))
        (i32.const 8))
      (i64.const 0))
    (i64.store offset=32
      (local.get $l0)
      (i64.const 0))
    (i64.store
      (i32.add
        (i32.add
          (local.get $l0)
          (i32.const 64))
        (i32.const 24))
      (i64.const 0))
    (i64.store
      (i32.add
        (i32.add
          (local.get $l0)
          (i32.const 64))
        (i32.const 16))
      (i64.const 0))
    (i64.store
      (i32.add
        (i32.add
          (local.get $l0)
          (i32.const 64))
        (i32.const 8))
      (i64.const 0))
    (i64.store offset=64
      (local.get $l0)
      (i64.const 0))
    (call $ethereum_getAddress
      (local.get $l0))
    (call $ethereum_storageStore
      (i32.add
        (local.get $l0)
        (i32.const 32))
      (local.get $l0))
    (call $ethereum_storageLoad
      (i32.add
        (local.get $l0)
        (i32.const 32))
      (i32.add
        (local.get $l0)
        (i32.const 64)))
    (global.set $g0
      (i32.add
        (local.get $l0)
        (i32.const 96))))
  (table $__indirect_function_table (export "__indirect_function_table") 1 1 anyfunc)
  (memory $memory (export "memory") 16)
  (global $g0 (mut i32) (i32.const 1048576))
  (global $__heap_base (export "__heap_base") i32 (i32.const 1048576))
  (global $__data_end (export "__data_end") i32 (i32.const 1048576)))
