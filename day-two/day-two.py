
def add_numbers(x,y):
    return (int(x)+int(y))

def multiply_numbers(x,y):
    return(int(x)*int(y))

def main():
    with open('intcode.txt','r') as file:
        raw_intcode = file.readlines()
    intcode = raw_intcode[0].split(',')
    intcode[-1] = intcode[-1].rstrip()
    opcode = intcode[0]
    i = 0
    while opcode != '99':
        pos_1 = int(intcode[i+1])
        pos_2 = int(intcode[i+2])
        pos_3 = int(intcode[i+3])
        if opcode == '1':
            final = str(add_numbers(intcode[pos_1],intcode[pos_2]))
            intcode[pos_3] = final
        if opcode == '2':
            final = str(multiply_numbers(intcode[pos_1],intcode[pos_2]))
            intcode[pos_3] = final
        i = i + 4
        opcode = intcode[i]
    print(intcode)
if __name__ == "__main__":
    main()
