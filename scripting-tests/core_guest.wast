(module
  (func $add-sub (param $a i32) (param $b i32) (result i32) (result i32)
        (local $c i32)
        local.get $a
        local.get $b
        i32.sub

        local.set $c
        
        local.get $a
        local.get $b
        i32.add
        local.get $c
        )

  (export "add-sub" (func $add-sub))
  )
