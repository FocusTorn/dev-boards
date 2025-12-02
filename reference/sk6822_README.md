
# PROJECT: SK6812 CONTROLLER



The hardware details and supporting documents are contained in this document.


## HARDWARE


  * Host Platform:
    * Name:         Raspberry Pi 4b
    * OS:           DietPi
    * From:         https://www.amazon.com/gp/product/B07TC2BK1X/ref=ppx_yo_dt_b_search_asin_title?ie=UTF8&th=1
    * DataSheet:    https://datasheets.raspberrypi.com/rpi4/raspberry-pi-4-datasheet.pdf

  * Slave
    * Name:         ESP32-S3 Series
    * From:         https://lonelybinary.com/en-us/products/s3?variant=43784065712285
    * DataSheet:    https://cdn.shopify.com/s/files/1/0331/9994/7908/files/ESP32_S3_Datasheet.pdf?v=1684451744

  * Level Shifter:
    * Name:         4 Channel 3.3V to 5V Bi-Directional I2C Logic Level Shifter Module
    * From:         https://www.amazon.com/dp/B0CRKC2BTY?psc=1&ref=ppx_yo2ov_dt_b_product_details#customerReviews
    * DataSheet:    https://www.tme.eu/Document/3d637fa1410688f9bd1aa00171908361/POLOLU-2595.pdf

  * Power Supply:
    * Name:         MEANWELL LRS-100-5, 5V 18A Switching Power Supply from:
    * From:         https://www.amazon.com/dp/B018TEAPRQ?psc=1&ref=ppx_yo2ov_dt_b_product_details
    * DataSheet:    https://www.mouser.com/datasheet/2/260/mwec_s_a0011714173_1-2274466.pdf

  * Led Strip:
    * Name:         5V DC SK6812 RGBW(RGBNW Natural White) from:
    * From:         https://www.amazon.com/dp/B01N2PC9KK?psc=1&ref=ppx_yo2ov_dt_b_product_details
    * DataSheet:    https://cdn-shop.adafruit.com/product-files/2757/p2757_SK6812RGBW_REV01.pdf



    ┏━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━┓              
    ┃             SK6812 RGBW - Timing              ┃
    ┣━━━━━━━━┯━━━━━━━━━━┯━━━━━━━━━┯━━━━━━━━━━━━━━━━━┫
    ┃  TYPE      CODE      LEVEL         TIMING     ┃
    ┠────────┼──────────┼─────────┼─────────────────┨
    ┃ T0H    │ 0        │ high    │ 0.3µs ±0.15µs   ┃
    ┠────────┼──────────┼─────────┼─────────────────┨
    ┃ T0L    │ 0        │ low     │ 0.9µs ±0.15µs   ┃
    ┠────────┼──────────┼─────────┼─────────────────┨
    ┃ T1H    │ 1        │ high    │ 0.6µs ±0.15µs   ┃
    ┠────────┼──────────┼─────────┼─────────────────┨
    ┃ T1L    │ 1        │ low     │ 0.6µs ±0.15µs   ┃
    ┠────────┼──────────┼─────────┼─────────────────┨
    ┃ Trst   │ reset    │ low     │ 80µs            ┃
    ┠────────┴──────────┴─────────┴─────────────────┨
    ┃ Data transmission time: TH+TL= 1.25µs ±600ns  ┃
    ┗━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━┛


## Extras

  * Arduino - Nano
    * Name:         Arduino - Nano
    * Pinout        https://docs.arduino.cc/resources/pinouts/A000005-full-pinout.pdf
    * DataSheet     https://docs.arduino.cc/resources/datasheets/A000005-datasheet.pdf

  * Fx2 based Logic Analyzer:
    * Name:         HiLetgo 24MHz USB Logic Analyzer
    * From:         https://www.amazon.com/dp/B077LSG5P2?ref=ppx_yo2ov_dt_b_fed_asin_title
    * Usage:        https://learn.sparkfun.com/tutorials/using-the-usb-logic-analyzer-with-sigrok-pulseview/all
    * GUI:          https://sigrok.org/wiki/PulseView






    ┌ ─ ┬ ─ ┐       ┏ ━ ┳ ━ ┓           ┍ ┎      ┭ ┮  ┯  ┰  ┱ ┲      ┑ ┒
                                        
    │   │   │       ┃   ┃   ┃           
                                        ┟ ┞         ╆ ╄  ╃ ╅         ┦ ┧
                                        
    ├ ─ ┼ ─ ┤       ┣ ━ ╋ ━ ┫           ┝ ┠    ╈ ╇  ┽ ┾  ╉ ╊  ╈ ╇    ┨ ┥
                                     
                                        ┢ ┡         ╀ ╁  ┿ ╂         ┩ ┪       
    │   │   │       ┃   ┃   ┃           
                                        
    └ ─ ┴ ─ ┘       ┗ ━ ┻ ━ ┛           ┕ ┖      ┵ ┶  ┷  ┸  ┹ ┺      ┙ ┚                
                                        
                                        ╭ ╮ ╯ ╰ ╶╷╵
                                        

    

    
    
How can I modify this to send the

~~~
import pigpio
import time

# GPIO pin for data signal (BCM 18)
LED_GPIO = 18

# SK6812 timing in microseconds
T0H = 0.3  # High for 0
T0L = 0.9  # Low for 0
T1H = 0.6  # High for 1
T1L = 0.6  # Low for 1
TRST = 80  # Reset low time

def sk6812_send_color(pi, gpio, colors):
    """
    Sends RGBW data to SK6812 LEDs.

    Args:
        pi: PIGPIO instance.
        gpio: GPIO pin used for LED data signal.
        colors: List of RGBW tuples (R, G, B, W), each 8-bit (0-255).
    """
    # Reset waveform
    pi.set_mode(gpio, pigpio.OUTPUT)
    pi.wave_clear()  # Clear existing waveforms

    # Create a list to hold waveform pulses
    pulses = []

    def add_bit(bit):
        """Adds waveform pulses for a single bit."""
        if bit == 0:
            pulses.append(pigpio.pulse(1 << gpio, 0, int(T0H * 1000)))
            pulses.append(pigpio.pulse(0, 1 << gpio, int(T0L * 1000)))
        else:
            pulses.append(pigpio.pulse(1 << gpio, 0, int(T1H * 1000)))
            pulses.append(pigpio.pulse(0, 1 << gpio, int(T1L * 1000)))

    # Encode the color data
    for color in colors:
        for byte in color:  # Encode R, G, B, W
            for i in range(8):  # MSB first
                add_bit((byte >> (7 - i)) & 1)

    # Add reset pulse
    pulses.append(pigpio.pulse(0, 1 << gpio, int(TRST * 1000)))

    # Create and send the waveform
    pi.wave_add_generic(pulses)
    wave_id = pi.wave_create()
    pi.wave_send_once(wave_id)

    # Wait for waveform to finish
    while pi.wave_tx_busy():
        time.sleep(0.001)

    # Cleanup
    pi.wave_clear()

# Initialize PIGPIO
pi = pigpio.pi()
if not pi.connected:
    exit(1)

# Example usage: Send RGBW values to 2 LEDs
colors = [
    (255, 0, 0, 0),   # Red
    (0, 255, 0, 0)    # Green
]
try:
    sk6812_send_color(pi, LED_GPIO, colors)
finally:
    pi.stop()

~~~


to send these pulse timings

~~~

    ┌───────────────────────────────────────────────┐              
    │             SK6812 RGBW - Timing              │
    ├───────────────────────────────────────────────┤
    │  TYPE      CODE      LEVEL         TIMING     │
    ├────────┼──────────┼─────────┼─────────────────┤
    │ T0H    │ 0        │ high    │ 0.3µs ±0.15µs   │
    ├────────┼──────────┼─────────┼─────────────────┤
    │ T0L    │ 0        │ low     │ 0.9µs ±0.15µs   │
    ├────────┼──────────┼─────────┼─────────────────┤
    │ T1H    │ 1        │ high    │ 0.6µs ±0.15µs   │
    ├────────┼──────────┼─────────┼─────────────────┤
    │ T1L    │ 1        │ low     │ 0.6µs ±0.15µs   │
    ├────────┼──────────┼─────────┼─────────────────┤
    │ Trst   │ reset    │ low     │ 80µs            │
    ├────────┴──────────┴─────────┴─────────────────┤
    │ Data transmission time: TH+TL= 1.25µs ±600ns  │
    └───────────────────────────────────────────────┘

~~~

    

    

    



    
## Potential Hurdles
* 
    * Getting the pulse timings to be less than a microsend
    
    * LIBRARY SPECIFIC:
        * pigpio
            - pigpio and most of the prebuilt libraries conflict
            * Certain methods will not send sub-μs pulses
                - set_servo_pulsewidth
                - set_PWM_dutycycle 
             
    
    
            
    
    
     

    
     
   
    
     
     
        
     
       
      
         
      
    
    
           
        
        
        
        
    
     
    
      
    
     
      
     
        

    
    





~~~


can you modify this code to light the led strip


~~~ c

/*
Compile and link

gcc -I/opt/vc/include -L/opt/vc/lib -lbcm_host -o nanopulse nanopulse.c

Run example (10000 10 nanosecond pulses with 2000 nano second gap)

sudo ./nanopulse 10 10000 2000
*/

#include <stdio.h>
#include <stdlib.h>
#include <stdint.h>
#include <fcntl.h>
#include <time.h>
#include <sys/mman.h>
#include <sys/stat.h>
#include <sys/types.h>


#include <bcm_host.h> 
#include <bits/mman-map-flags-generic.h>
#include <asm-generic/mman.h>
#define PI_BASE_ADDR  bcm_host_get_peripheral_address()
#define GPIO_BASE     (PI_BASE_ADDR + 0x200000)
#define PWM_BASE      (PI_BASE_ADDR + 0x20C000)
#define CLK_BASE      (PI_BASE_ADDR + 0x101000)

#define CLK_LEN   0xA8
#define GPIO_LEN  0xB4
#define PWM_LEN   0x28

#define PWM_CTL      0
#define PWM_STA      1
#define PWM_RNG1     4
#define PWM_FIFO     6

#define PWM_CTL_CLRF1 (1<<6)
#define PWM_CTL_USEF1 (1<<5)
#define PWM_CTL_MODE1 (1<<1)
#define PWM_CTL_PWEN1 (1<<0)

#define PWM_STA_EMPT1 (1<<1)

#define CLK_PASSWD  (0x5A<<24)

#define CLK_CTL_MASH(x)((x)<<9)
#define CLK_CTL_BUSY    (1 <<7)
#define CLK_CTL_KILL    (1 <<5)
#define CLK_CTL_ENAB    (1 <<4)
#define CLK_CTL_SRC(x) ((x)<<0)

#define CLK_CTL_SRC_PLLD 6  /* 500.0 MHz */

#define CLK_DIV_DIVI(x) ((x)<<12)
#define CLK_DIV_DIVF(x) ((x)<< 0)

#define CLK_PWMCTL 40
#define CLK_PWMDIV 41

#define MAX_BITS 224

typedef struct
{
   unsigned divider;
   unsigned bits;
} pwm_clock_cfg_t;

unsigned base_nano[]={4, 8, 10, 20, 40, 80, 100, 200, 250, 500, 1000};

static volatile uint32_t  *clkReg  = MAP_FAILED;
static volatile uint32_t  *gpioReg = MAP_FAILED;
static volatile uint32_t  *pwmReg  = MAP_FAILED;

static void mynanosleep(unsigned nanos)
{
   struct timespec ts, tr;

   ts.tv_sec = 0;
   ts.tv_nsec = nanos;
   while (nanosleep(&ts, &tr))
   {
      ts = tr;
   }
}

int gpioSetMode(unsigned gpio, unsigned mode)
{
   int reg, shift;

   reg   =  gpio/10;
   shift = (gpio%10) * 3;

   gpioReg[reg] = (gpioReg[reg] & ~(7<<shift)) | (mode<<shift);

   return 0;
}

int gpioGetMode(unsigned gpio)
{
   int reg, shift;

   reg   =  gpio/10;
   shift = (gpio%10) * 3;

   return (*(gpioReg + reg) >> shift) & 7;
}

static void initPWM(unsigned divider)
{
   /* reset PWM clock */
   clkReg[CLK_PWMCTL] = CLK_PASSWD | CLK_CTL_KILL;

   mynanosleep(10000);

   /* set PWM clock source as 500 MHz PLLD */
   clkReg[CLK_PWMCTL] = CLK_PASSWD | CLK_CTL_SRC(CLK_CTL_SRC_PLLD);

   mynanosleep(10000);

   /* set PWM clock divider */
   clkReg[CLK_PWMDIV] = CLK_PASSWD | CLK_DIV_DIVI(divider) | CLK_DIV_DIVF(0);

   mynanosleep(10000);

   /* enable PWM clock */
   clkReg[CLK_PWMCTL] =
      CLK_PASSWD | CLK_CTL_ENAB | CLK_CTL_SRC(CLK_CTL_SRC_PLLD);

   mynanosleep(100000);

   /* reset PWM */
   pwmReg[PWM_CTL] = 0;

   /* clear PWM status bits */
   pwmReg[PWM_STA] = -1;

   mynanosleep(10000);
}

static void sendPulse(unsigned bits)
{
   int i;
   uint32_t word;

   if      (bits == 0)       bits = 1;
   else if (bits > MAX_BITS) bits = MAX_BITS;

   /* clear PWM fifo */

   pwmReg[PWM_CTL] = PWM_CTL_CLRF1;

   mynanosleep(10000);

   while (bits >= 32)
   {
      pwmReg[PWM_FIFO] = -1;
      bits -= 32;
   }

   if (bits)
   {
      word = 0;

      for (i=0; i<bits; i++) word |= (1<<(31-i));

      pwmReg[PWM_FIFO] = word;
   }

   pwmReg[PWM_FIFO] = 0;

   /* enable PWM for serialised data from fifo */
   pwmReg[PWM_CTL] = PWM_CTL_USEF1 | PWM_CTL_MODE1 | PWM_CTL_PWEN1;
}


static uint32_t * mapMem(int fd, unsigned base, unsigned len)
{
   return mmap
   (
      0,
      len,
      PROT_READ|PROT_WRITE|PROT_EXEC,
      MAP_SHARED|MAP_LOCKED,
      fd,
      base
   );
}

pwm_clock_cfg_t getDivBits(unsigned nano)
{
   pwm_clock_cfg_t cfg;

   unsigned i, base, bits, err, bestErr, bestBase, bestBits;

   bestErr = -1;

   for (i=0; i<sizeof(base_nano)/sizeof(unsigned);i++)
   {
      bits = nano / base_nano[i];

      if (bits > MAX_BITS) bits = MAX_BITS;

      err = nano - (bits * base_nano[i]);

      if (err < bestErr)
      {
         bestErr = err;
         bestBase = base_nano[i];
         bestBits = bits;
      }
   }

   cfg.divider = bestBase / 2;
   cfg.bits = bestBits;

   return cfg;
}

int main(int argc, char *argv[])
{
   int fd, i, gpio, mode;
   pwm_clock_cfg_t cfg;

   int nanos=1000, pulses=100, gap=5000;

   fd = open("/dev/mem", O_RDWR | O_SYNC);

   if (fd<0)
   {
      printf("need to run as root, e.g. sudo %s\n", argv[0]);
      exit(1);
   }

   gpioReg = mapMem(fd, GPIO_BASE, GPIO_LEN);
   pwmReg  = mapMem(fd, PWM_BASE,  PWM_LEN);
   clkReg  = mapMem(fd, CLK_BASE,  CLK_LEN);

   close(fd);

   if (argc > 1) nanos  = atoi(argv[1]);
   if (argc > 2) pulses = atoi(argv[2]);
   if (argc > 3) gap    = atoi(argv[3]);

   nanos = (nanos * 4) / 3;

   if      (nanos < 4)      nanos = 4;
   else if (nanos > 224000) nanos = 224000;

   if (pulses < 1) pulses = 1;

   if (gap < 0) gap = 0;

   cfg = getDivBits(nanos);

   printf("%d pulses of %d nanos with gap of %d nanos (div=%d bits=%d)\n",
      pulses, cfg.divider * 2 * cfg.bits *3 / 4, gap, cfg.divider, cfg.bits);

   mode = gpioGetMode(18); /* save original mode */

   gpioSetMode(18, 2); /* set to ALT5, PWM1 */

   initPWM(cfg.divider);

   for (i=0; i< pulses; i++)
   {
      sendPulse(cfg.bits);

      mynanosleep(nanos + gap);
   }

   gpioSetMode(18, mode); /* restore original mode */
}
~~~



