mov x1, #1
mov x0, #1
mov x2, #1
mov x3, #10
mul x0, x1, x0
add x1, x1, x2
cmp x1, x3
b.lt #4