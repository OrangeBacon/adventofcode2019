entry:
    $input = 0
    $input10 = 0
    $isEq17 = 0
    inp $input
    add 10 $input $input10
    out $input10
    out @0
    rba 1
    out @1
    rba -1
    eql 17 $input10 $isEq17
    jnz $isEq17 :eq17
    jez $isEq17 :neq17
    hlt

eq17:
    out 80085
    jmp :entry

neq17:
    $isEq1 = 0
    eql 1 $input $isEq1
    jnz $isEq1 :exit
    out 666
    jez 0 :entry

exit: