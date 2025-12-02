#!/usr/bin/env python

import time
import bme680Mine
import fileinput
import sys

try:
    sensor = bme680Mine.BME680(bme680Mine.I2C_ADDR_PRIMARY)
except (RuntimeError, IOError):
    sensor = bme680Mine.BME680(bme680Mine.I2C_ADDR_SECONDARY)


#====|| Start: OVER RIDES ||================================================================================================================================================= 
#============================================================================================================================================================================ 
#
# These settings can be tweaked to change the balance between accuracy and noise in the data.
#

sensor.set_humidity_oversample(bme680Mine.OS_2X)
sensor.set_pressure_oversample(bme680Mine.OS_4X)
sensor.set_temperature_oversample(bme680Mine.OS_8X)
sensor.set_filter(bme680Mine.FILTER_SIZE_3)
sensor.set_gas_status(bme680Mine.ENABLE_GAS_MEAS)

sensor.set_gas_heater_temperature(320)
sensor.set_gas_heater_duration(150)
sensor.select_gas_heater_profile(0)





#============================================================================================================================================================================
#====|| End: OVER RIDES ||=================================================================================================================================================== 

def replaceAll(file,searchExp,replaceExp):
    for line in fileinput.input(file, inplace=1):
        if searchExp in line:
            line = 'aqs_avg = {0:.2f}'.format(sensor.get_aqs_avg())
            # line = line.replace(searchExp,replaceExp)
        sys.stdout.write(line)

print("Setting intial data baselines...")
sensor.set_baselines(1)
print('\nBaselines calculated - (Hum BL: {1:.2f}) (Gas BL: {0})\n'.format( sensor.get_gas_baseline(), sensor.get_hum_baseline() ))


# pkill -f script.py
try:
    while True:
        sensor.set_air_quality_score(records_to_average=2, verbose=False)


        
        replaceAll("/home/pi/printer_data/config/saved_variables.cfg","aqs_avg","Goodbye\sWorld.")

        print(sensor.aqs_avg)
        time.sleep(1)

except EOFError:
    pass 
    
    
#<