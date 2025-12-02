#!/usr/bin/env python

# /home/pi/.local/lib/python3.9/site-packages/bme680Mine


import time
import For_Klipper.bme680.bme680 as bme680Mine




try:
	sensor = bme680Mine.BME680(bme680Mine.I2C_ADDR_PRIMARY)
except (RuntimeError, IOError):
	sensor = bme680Mine.BME680(bme680Mine.I2C_ADDR_SECONDARY)


# import sys
# print()
# print(sys.path)

# sys.path.append("/home/mylinux/python-packages")

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

#====|| Start: CALCULATE GAS BASELINE ||===================================================================================================================================== 
#============================================================================================================================================================================ 

print("Setting intial data baselines")
sensor.set_baselines(1)

gas_baseline = sensor.get_gas_baseline()
hum_baseline = sensor.get_hum_baseline()

if not gas_baseline:
	raise ValueError('gas_baseline not calculated')
	
if not hum_baseline:
	raise ValueError('hum_baseline not calculated')
	
print('\nBaselines calculated - (Hum BL: {1:.2f}) (Gas BL: {0})\n'.format( gas_baseline, hum_baseline ))

#============================================================================================================================================================================
#====|| End: CALCULATE GAS BASELINE ||======================================================================================================================================= 



def get_aqs():
	sensor.get_air_quality_score(verbose=false)



# start_time = time.time()
# curr_time = time.time()

# air_quality_data = []

# try:
# 	while True:
# 		if sensor.get_sensor_data() and sensor.data.heat_stable:
# 			curr_time = time.time()
			
# 			# Readability variables 
# 			gas = sensor.data.gas_resistance
# 			hum = sensor.data.humidity
						
# 			# Calculate gas and humidity offsets 
# 			gas_offset = gas_baseline - gas
# 			hum_offset = hum - hum_baseline
							
# 			# Calculate hum_score2
# 			if hum_offset > 0:
# 				hum_score = (100 - hum_baseline - hum_offset) / (100 - hum_baseline) * (hum_weighting * 100)
# 			else:
# 				hum_score = (hum_baseline + hum_offset) / hum_baseline * (hum_weighting * 100)

# 			# Calculate gas_score 
# 			if gas_offset > 0:
# 				gas_score = (gas / gas_baseline)
# 				gas_score *= (100 - (hum_weighting * 100))
# 			else:
# 				gas_score = 100 - (hum_weighting * 100)

			
# 			# Calculate air_quality_score.
# 			air_quality_score = hum_score + gas_score 	


# 			print(air_quality_score)


# 			air_quality_data.append( air_quality_score )
			
# 			#print('{:.3f} - {}'.format( curr_time-start_time, len( air_quality_data ) ) )
			
# 			if len( air_quality_data ) == 30:
# 				air_quality_score_avg = sum(air_quality_data[-30:]) / 30
# 				print(air_quality_score_avg)
# 				air_quality_data.clear()
			
			
# 			# print('{0:.2f} - Gas: {1:.5f} Ohms, Hum: {2:.2f}, Qual: {3:.2f}'.format(curr_time - start_time,gas, hum, air_quality_score))

# 			time.sleep(1)
		
# except KeyboardInterrupt:
# 	pass

