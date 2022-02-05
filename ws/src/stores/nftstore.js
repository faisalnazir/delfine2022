import {writable} from 'svelte/store'

export const nftdata = writable([])

const fetchNFTS = async () => {
  const url = "https://deep-index.moralis.io/api/v2/0x5D6abAA8D3D615727a3503dd7aA053724297bA68/nft?chain=mumbai&format=decimal"

  const res = await fetch(url, {
    method: 'GET',
    headers: {
      'accept': 'application/json',
      'x-api-key': '685JUEmiN5WrZcHpqxlQ0qEsY99J4Ydqp0RjsL3pWuOU9nzXkuhDg6b9X5OWxX41'
    }
  })
  // console.log(res)
  const data = await res.json()
  nftdata.set(data.result)
  // console.log(data.result)

};
fetchNFTS()
