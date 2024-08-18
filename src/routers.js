import { createRouter, createWebHistory } from "vue-router";

import NotesPage from "./views/NotesPage.vue";
import CreateNotePage from "./views/CreateNotePage.vue";
import NoteDetail from "./views/NoteDetail.vue";

const routes = [
  {
    path: "/notes-page",
    name: "NotesPage",
    component: NotesPage,
  },
  {
    path: "/create-note-page",
    name: "CreateNotePage",
    component: CreateNotePage,
  },
  {
    path: "/note-detail",
    name: "NoteDetail",
    component: NoteDetail,
  },
];

const router = createRouter({
  history: createWebHistory(),
  routes,
});

export default router;
