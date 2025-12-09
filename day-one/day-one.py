def main():
    with open('mass.txt','r') as file:
        masses = file.readlines()
    fuels=[]
    for mass in masses:
        final_fuel = 0
        fuel = int((int(mass)/3))-2
        while fuel > 0:
            final_fuel += fuel
            fuel = int((int(fuel)/3))-2
        fuels.append(final_fuel)
    total_fuel = 0
    for fuel in fuels:
        total_fuel += fuel    

    print(total_fuel)

if __name__ == "__main__":
    main()
