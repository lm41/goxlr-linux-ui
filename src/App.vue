<template>
  <HelloWorld />
</template>

<script>
import HelloWorld from './components/HelloWorld.vue'

export default {
  name: 'App',
  components: {
    HelloWorld
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
