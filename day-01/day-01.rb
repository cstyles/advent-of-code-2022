a=STDIN.readlines.map(&:to_i).chunk{_1!=0}.map{_2.sum}.sort
p a[-1],a[-3..].sum
