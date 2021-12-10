<template>
  <v-container>
    <v-row>
      <v-col cols="6">
        <v-card style="height: 650px; margin: 10px">
          <v-img style="height: 650px" :src="url" />
        </v-card>
      </v-col>
      <v-col cols="6">
        <v-card style="height: 650px; margin: 10px">
          <h1 class="text-item">
            教科書名：{{ textName }}
            <br />
            学部：{{ major }}
            <br />
            大学：{{ uniName }}
            <br />
            商品の状態：{{ status }}
            <br />
            商品の説明：{{ explain }}
            <br />
            出品者：{{ sellerUni }},{{ sellerName }}
          </h1>
          <h2>チャット</h2>
          <chat :name="name" :uid="uid" :url="avatarUrl" />
        </v-card>
      </v-col>
    </v-row>
  </v-container>
</template>

<script>
import { db } from "../main.js";
import { collection } from "firebase/firestore";
import { getDocs } from "firebase/firestore";
import { doc } from "firebase/firestore";

import { getDoc } from "firebase/firestore";
import { getAuth } from "firebase/auth";

//imageのimport
import { getStorage } from "firebase/storage";
import { getDownloadURL, ref } from "firebase/storage";

import chat from "../components/chat.vue";
export default {
  data() {
    return {
      explain: null,
      good: null,
      major: null,
      status: null,
      textName: null,
      uniName: null,
      sellerName: null,
      sellerUni: null,
      url: null,
      message: "",
      uid: null,
      name: "",
      avatarUrl: "",
    };
  },

  components: { chat },

  async created() {
    //const selectedText = this.$store.state.detailText;
    const selectedText = "AnrpPw5zb7yPlpWZocgF";

    console.log(selectedText);
    const docRef = doc(db, "text", selectedText);
    const docSnap = await getDoc(docRef);
    this.explain = docSnap.data().explain;
    this.good = docSnap.data().good;
    this.major = docSnap.data().major;
    this.status = docSnap.data().status;
    this.textName = docSnap.data().textName;
    this.uniName = docSnap.data().uniName;
    this.uid = docSnap.data().userID;
    const fileName = docSnap.data().fileName;
    const getUser = doc(db, "users", this.uid);
    const UserDetail = await getDoc(getUser);
    this.sellerName = UserDetail.data().name;
    this.sellerUni = UserDetail.data().university;

    const storage = getStorage();
    await getDownloadURL(ref(storage, this.uid + "/" + fileName)).then(
      (url) => {
        this.url = url;
      }
    );

    const docSnap_1 = await getDocs(collection(db, "messages"));
    this.datas = docSnap_1.docs;

    const auth = getAuth();
    const user = auth.currentUser;
    const docRef_1 = doc(db, "users", user.uid);
    const docS = await getDoc(docRef_1);
    this.name = docS.data().name;

    await getDownloadURL(ref(storage, user.uid + "/" + "avatar")).then(
      (url) => {
        this.avatarUrl = url;
      }
    );
  },

  methods: {},
};
</script>

<style scoped>
.text-item {
  text-align: center;
}
</style>
