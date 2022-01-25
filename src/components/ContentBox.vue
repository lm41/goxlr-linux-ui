<template>
  <div class="content">
    <div class="contentTitle">{{ title }}</div>
    <div class="contentBody">
      <slot />
    </div>
  </div>
</template>

<script>
export default {
  name: "ContentBox",

  props: {
    title: String,
  }
}
</script>

<style scoped>
.content {
  margin-left: 15px;
  display: inline-flex;
  flex-direction: column;
  flex-wrap: nowrap;
  background-color: #2d3230;
  padding: 20px;
}

.contentTitle {
  color: #fff;
  margin-bottom: 10px;
  text-transform: uppercase;
}

.contentBody {
  display: inline-flex;
  flex-direction: row;

  /** TODO: Apply this to children, not here! **/
}
</style>