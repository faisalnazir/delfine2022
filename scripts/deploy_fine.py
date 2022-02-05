import os
from brownie import FINE, accounts, network, config

airtable_key = 'keyf32lC4wKuwmbvr'


def main():
    dev = accounts.load('matic')
    fine = FINE.deploy(airtable_key, {'from': dev})

    artworkIds = [123985, 137298, 123432,
                  123703, 124165, 123967, 133460, 123434, 133447, 124286, 124167, 117918, 133448, 98300, 150188, 123433, 124001, 124002, 97193, 124005, 128300, 95046, 98647, 126088, 159535, 95058, 95149, 144668, 157216, 157667, 163512, 423612, 423613, 123982, 125573, 125575, 123992, 124429, 126239, 124008]

    artworkQty = [1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1,
                  1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1]

    fine.mintBatch(dev, artworkIds, artworkQty, hex(786), {'from': dev})

    return fine
