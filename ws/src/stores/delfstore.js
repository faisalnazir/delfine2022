import {writable} from 'svelte/store';

export const delfinedata = writable([]);

const fetchDelfine = async () => {
  const gurl = 'https://openaccess-api.clevelandart.org/api/artworks/?department=European%20Painting%20and%20Sculpture&limit=100';
  const url = 'https://openaccess-api.clevelandart.org/api/artworks/?department=Islamic%20Art&limit=40';


  const res = await fetch(url);
  // console.log(res);
  const data = await res.json();
  // console.log(data.data)
  delfinedata.set(data.data);
};

// const fetchDelfine = async () => {
// const url = 'https://pokeapi.co/api/v2/pokemon?limit=150';
// const res = await fetch(url);
// // console.log(res);
// const data = await res.json();
// // console.log(data.results);
// const loadedPokemon = data.results.map((data, index) => {
//     return {
//       name: data.name,
//       id: index + 1,
//       image: `https://picsum.photos/id/${
//         index + 1
//       }/100/100`
//     };
// });
// // console.log(loadedPokemon)
// delfinedata.set(loadedPokemon);
// };


fetchDelfine();
