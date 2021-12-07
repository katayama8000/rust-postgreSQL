<template>
  <v-container>
    <v-row>
      <v-col cols="6">
        <v-card style="height: 650px; margin: 10px">
          <v-img style="height: 650px" :src="url" />
        </v-card>
      </v-col>
      <v-col cols="6">
        <v-card style="height: 650px; margin: 10px; ">
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
          <chat :name="name" :uid="uid" :url="avatarUrl"/>
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
import { getDownloadURL,ref } from "firebase/storage";

import chat from "../components/chat.vue"
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
      name:"",
      avatarUrl:""
    };
  },

  components:{chat},

  async created() {
    //const selectedText = this.$store.state.detailText;
    const selectedText = "AnrpPw5zb7yPlpWZocgF"

    console.log(selectedText);
    const docRef = doc(db, "text", selectedText);
    const docSnap = await getDoc(docRef);
    this.explain =
      docSnap._document.data.value.mapValue.fields.explain.stringValue;
    this.good = docSnap._document.data.value.mapValue.fields.good.stringValue;
    this.major = docSnap._document.data.value.mapValue.fields.major.stringValue;
    this.status =
      docSnap._document.data.value.mapValue.fields.status.stringValue;
    this.textName =
      docSnap._document.data.value.mapValue.fields.textName.stringValue;
    this.uniName =
      docSnap._document.data.value.mapValue.fields.uniName.stringValue;
    this.uid = docSnap._document.data.value.mapValue.fields.userID.stringValue;
    const fileName =
      docSnap._document.data.value.mapValue.fields.fileName.stringValue;
    const getUser = doc(db, "users", this.uid);
    const UserDetail = await getDoc(getUser);
    this.sellerName =
      UserDetail._document.data.value.mapValue.fields.name.stringValue;
    this.sellerUni =
      UserDetail._document.data.value.mapValue.fields.university.stringValue;

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
    this.name = docS._document.data.value.mapValue.fields.name.stringValue;

    await getDownloadURL(ref(storage, user.uid + "/" + "avatar")).then(
      (url) => {
        this.avatarUrl = url
      }
    );
  },

  methods: {
  },
};
</script>

<style scoped>
.text-item{
  text-align: center

}
</style>
