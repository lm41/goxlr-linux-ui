<template>
  <HelloWorld />
</template>

<script>
import HelloWorld from './components/HelloWorld.vue'
import { invoke } from '@tauri-apps/api/tauri'


export default {
  name: 'App',
  components: {
    HelloWorld
  },

  created() {
   //  // Attempt a call through to rust..
   invoke('test_command', { param: "Hello From javascript!" })
   .then((res) => console.log(res) );


  }
}
</script>

<style>

body {
  background-color: #252927;
}

@font-face {
  src: url('./assets/LeagueMono.ttf');
  font-family:'LeagueMonoVariable';
  font-style: normal;
  font-stretch: 1% 500%; /* Required by Chrome */
}

#app {
  /*font-family: Avenir, Helvetica, Arial, sans-serif;*/
  font-family: LeagueMonoVariable, sans-serif;

  -webkit-font-smoothing: antialiased;
  -moz-osx-font-smoothing: grayscale;
  text-align: center;
  color: #2c3e50;
  margin-top: 20px;
  user-select: none;
}
</style>
