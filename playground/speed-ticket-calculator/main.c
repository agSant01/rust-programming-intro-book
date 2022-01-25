#include <stdio.h>
#include <stdbool.h>

typedef enum SpeedingTicket
{
    NoTicket = 0,
    RegularTicket = 1,
    BigTicket = 2,
} SpeedingTicket;

const int SPEED_LIMIT = 60;
const int EXCESSIVE_SPEED = 80;
const int BIRTHDAY_INCREASE = 5;

SpeedingTicket speedingCalculator(int speed, bool birthday)
{
    int inc = birthday ? BIRTHDAY_INCREASE : 0;
    if (speed > EXCESSIVE_SPEED + inc)
    {
        return BigTicket;
    }
    else if (speed > SPEED_LIMIT + inc)
    {
        return RegularTicket;
    }
    else
    {
        return NoTicket;
    }
}

void main()
{
    printf("Hello world!\n");
    SpeedingTicket r = speedingCalculator(60, false);
    printf("%d\n", r);
    r = speedingCalculator(65, false);
    printf("%d\n", r);
    r = speedingCalculator(65, true);
    printf("%d\n", r);
}