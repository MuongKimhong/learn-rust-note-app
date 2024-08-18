<template>
  <div
    class="py-4 px-3"
    id="note"
    @mouseenter="showDeleteBtn = true"
    @mouseleave="showDeleteBtn = false"
    @click="selectNote()"
  >
    <h2>{{ note.title }}</h2>

    <div class="text-right">
      <small class="mr-2">{{ note.created_at }}</small>
      <v-btn
        v-if="showDeleteBtn"
        size="x-small"
        class="text-capitalize"
        color="red"
        @click="deleteNoteBtnOnClick()"
      >
        <span class="mt-1">Delete</span>
      </v-btn>
    </div>
    <hr id="hr" />
  </div>
</template>

<script>
export default {
  name: "Note",
  props: {
    note: Object,
  },
  data() {
    return {
      showDeleteBtn: false,
    };
  },
  methods: {
    deleteNoteBtnOnClick: function () {
      this.$emit("deleteNoteBtnOnClick", this.note);
    },

    selectNote: function () {
      this.$router.push({
        path: "/note-detail",
        query: {
          data: JSON.stringify(this.note),
        },
      });
    },
  },
};
</script>

<style scoped>
#note:hover {
  background-color: #757575;
  border-radius: 8px;
  cursor: pointer;
}
#hr {
  color: #757575;
  background-color: #757575;
  border: none;
  height: 1px;
}
</style>
