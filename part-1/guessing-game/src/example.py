class sample:
    def __init__(self) -> None:
        self.x = 0 

class sample2:
    def __init__(self) -> None:
        self.x = "yo" 


def add_one_to_x(input):
    input.x = 1

 

yo = sample();

print(yo.x) 
add_one_to_x(yo) # add_one_to_x(&mut yo);
print(yo.x)

yow = sample2();
print(yow.x) 
add_one_to_x(yow) # add_one_to_x(&mut yo);
print(yow.x)