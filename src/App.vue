<template>
  <v-app id="inspire">
    <v-navigation-drawer v-model="drawer" app>
      <v-list rounded>
        <v-subheader>メニュー</v-subheader>
        <v-list-item-group color="primary">
          <div v-for="page in pages" :key="page.name">
            <router-link :to="page.url"><v-list-item>{{ page.name }}</v-list-item></router-link>
          </div>
        </v-list-item-group>
      </v-list>
    </v-navigation-drawer>
    <v-app-bar app>
      <v-app-bar-nav-icon @click="drawer = !drawer"></v-app-bar-nav-icon>
      <v-toolbar-title>教科書シェアアプリ</v-toolbar-title>
    </v-app-bar>
    <v-main>
      <router-view/>
    </v-main>
  </v-app>
</template>

<script>
export default {
  data() {
    return {
      drawer: null,
      pages: [
        {
          name: "TOP",
          url: "/top",
        },
        {
          name: "ログイン",
          url: "/login",
        },
        {
          name: "新規登録",
          url: "/register",
        },
        {
          name: "メイン",
          url: "/main",
        },
        {
          name: "詳細",
          url: "/detail",
        },
        {
          name: "出品",
          url: "/sell",
        },
        {
          name: "個人情報入力",
          url: "/personalinfo",
        },
        {
          name: "プロフィール",
          url: "/profile",
        },
        {
          name: "チャット",
          url: "/chat",
        },
        {
          name: "todo",
          url: "/todo",
        },
        {
          name: "時計",
          url: "/time",
        },
        {
          name: "test",
          url: "/about",
        },
        {
          name: "google",
          url: "/google",
        },
      ],
    };
  },

  computed:{
    isAuthenticated(){
      return this.$store.getters.idToken !== null
    }
  },
};
</script>
