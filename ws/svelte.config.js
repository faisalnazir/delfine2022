import preprocess from 'svelte-preprocess';
// import adapter from '@sveltejs/adapter-auto';
import adapter from '@sveltejs/adapter-static';

/** @type {import('@sveltejs/kit').Config} */
const config = {
  kit: {
    adapter: adapter(
      {pages: 'build', assets: 'build', fallback: null, precompress: false}
    ),

    // hydrate the <div id="svelte"> element in src/app.html
    target: '#svelte'
  },

  preprocess: [preprocess(
      {postcss: true}
    )]
};

export default config;
