<template>
  <div>
    <v-text-field
      variant="underlined"
      placeholder="Search"
      v-model="searchText"
      @input="handleSearchInputTyping"
    ></v-text-field>
  </div>
</template>

<script>
export default {
  name: "Search",
  data() {
    return {
      searchText: "",
    };
  },
  methods: {
    handleSearchInputTyping: function () {
      this.$emit("searchTyping", this.searchText);
    },
  },
};
</script>
