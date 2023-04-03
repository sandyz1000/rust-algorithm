"""
Algorithm: There are N cars in line at the gas station. The station has three fuel dispensers.

Write a custom catalog title here
There are N cars in line at the gas station. There are three tankers at the station, labeled X, Y, and Z. 
Each fuel dispenser contains a certain amount of fuel. At any time, the amount of available fuel is clearly 
displayed on each fuel dispenser.
When a car reaches the front of the queue, the driver can choose to drive to any distributor that is not 
occupied by another car. Suppose the fuel demand of this car is D litres. The driver must select a fuel 
dispenser with a fuel capacity of at least D liters. If the capacity of all unused dispensers is less than 
D liters, the driver must wait for other cars to finish refueling. If all the dispensers are not occupied 
and none are at least D litres, then the driver will not be able to refuel the car and will block the queue 
indefinitely. If there is at least D litre in more than one dispenser, the driver will choose the one with 
the smallest letter.
Every driver must wait for a while before starting to refuel the car. Calculate the longest waiting time 
between all drivers. Suppose it takes one second to refuel one liter of fuel, and the car in motion is instantaneous.

Write a function:
public static Integer solution(int[] A, int X, int Y, int Z)
X, Y, Z represent 3 fuel dispensers.
The position inside the array A represents the vehicle, and the element represents the amount of fuel required 
by the vehicle. Return the maximum waiting time of the car. If there is a car that cannot be refueled, the 
function returns -1.

If 1: X=7, Y=11, Z=3.
A[0]=2
A[1]=8
A[2]=4
A[3]=3
A[4]=2
The time that the car waits in the queue is 0, 0, 2, 2, and 8 seconds. The function returns 8.
If 2: X=4, Y=0, Z=3.
A[0]=5
The function returns -1.

"""

