# Sequence Diagrams
### The market maker does not give an offer:
![mm_no_offer](./diagrams/mm_no_offer.png)

### The market maker gives an offer, trader does not execute:
The Market Maker may interact directly with a wallet or contract.

![mm_offer_trader_doesnt_execute](./diagrams/mm_offer_trader_doesnt_execute.png)

### The market maker gives an offer, trader executes offer:
The Market Maker may interact directly with a wallet or contract.

![mm_offer_trader_execute](./diagrams/mm_offer_trader_execute.png)

### Trader creates a listing for market maker offer, trader does not execute:
Trader may want to sell their (created) options to the market maker instead.

![trader_listing_mm_offer_no_execute](./diagrams/trader_listing_mm_offer_no_execute.png)

### Trader creates a listing for market maker offer, trader executes:
Trader may want to sell their (created) options to the market maker instead.

![trader_listing_mm_offer](./diagrams/trader_listing_mm_offer.png)

### Market Maker General Pricing:
This sequence allows the front end to display the general pricing for assets before
requesting for a quote.
![mm_pricing](./diagrams/mm_pricing.png)

### General
#### Request for Quote (RFQ) JSON data structure
If the `listingId` is given then all other details will be ignored.

```json
{
  "listingId": 0,
  "traderAddress": "",
  "underlyingAsset": "",
  "underlyingAmount": 0,
  "exerciseAsset": "",
  "exerciseAmount": 0,
  "exerciseTimestamp": 0,
  "expiryTimestamp": 0,
  "settlementSeed": 0,
  "nextClaimNum": 0
}
```

#### Reply JSON data structure
`messageId` and `traderAddress` would just echo back the details in the
RFQ message.

If an offer was made:
```json
{
  "traderAddress": "",
  "messageId": 0,
  "hasOffer": true
}

```

If an offer was not made:
```json
{
  "traderAddress": "",
  "messageId": 0,
  "hasOffer": false
}
```