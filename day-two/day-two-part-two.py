def add_numbers(x,y):
    return (int(x)+int(y))

def multiply_numbers(x,y):
    return(int(x)*int(y))

def main():
    final_1 = 0
    final_2 = 0
    with open('intcode.txt','r') as file:
        raw_intcode = file.readlines()
    intcode = raw_intcode[0].split(',')
    intcode[-1] = intcode[-1].rstrip()
    for j in range(0,100):
        for k in range(0,100):
            i = 0
            intcode = raw_intcode[0].split(',')
            intcode[-1] = intcode[-1].rstrip()
            intcode[1] = str(j)
            intcode[2] = str(k)
            opcode = intcode[0]
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
                if intcode[0] == "19690720":
                    final_1 = j
                    final_2 = k
                    print(intcode)
                i = i + 4
                opcode = intcode[i]
    print("NOUN: " + str(final_1))
    print("VERB: " + str(final_2))
    print("Final: " + str(final_1) + str(final_2))
if __name__ == "__main__":
    main()

