<template>
  <v-container>
    <v-row>
      <v-col cols="11">
        <h1 class="text-center">メイン</h1>
      </v-col>
      <v-col cols="1">
        <router-link to="sell"><v-btn>出品</v-btn></router-link>
      </v-col>
      <v-btn @click="logout">logout</v-btn>
    </v-row>
    <br />
    <hr />
    <br />
    <v-row justify="center" class="text-center">
      <v-col cols="3" v-for="(data, index) in datas" :key="index">
        <v-card>
          <v-card-title class="headline"
            >{{
              data._document.data.value.mapValue.fields.textName.stringValue
            }}
            ({{
              data._document.data.value.mapValue.fields.uniName.stringValue
            }})</v-card-title
          >
          <v-card-subtitle>{{
            data._document.data.value.mapValue.fields.major.stringValue
          }}</v-card-subtitle>
          <v-divider class="mx-3"></v-divider>
          <v-card-text>
            <!--------------------------------- image ------------------------------------------>
            <v-img style="height: 330px" :src="data.url" />
          </v-card-text>
          <v-card-actions>
            <v-btn @click="toDetailPage(index)">（仮）詳細</v-btn>
            <v-spacer></v-spacer>
            <v-btn class="mx-2" fab dark small color="pink">
              <v-icon dark @click="addheart(index)"> mdi-heart</v-icon>
            </v-btn>
            {{ data._document.data.value.mapValue.fields.good.integerValue }}
          </v-card-actions>
        </v-card>
      </v-col>
    </v-row>
  </v-container>
</template>

<script>
import { collection } from "firebase/firestore";
import { getDocs } from "firebase/firestore";
//import { getDoc,setDoc } from "firebase/firestore";
import { doc, updateDoc } from "firebase/firestore";
import { db } from "../main.js";
//imageのimport
import { getStorage, ref } from "firebase/storage";
import { getDownloadURL } from "firebase/storage";
export default {
  name: "app",
  data() {
    return {
      datas: [],
      id: [],
      good: 0,
      url: [],
      fileNames: [],
      uids: [],
      docId:[]
    };
  },

  async created() {
    this.create()
  },

  computed:{
  },

  watch:{
    good:function(){
    }
  },

  methods: {
    toDetailPage(index) {
      this.$store.commit("upDataDetailText", this.datas[index].id);
      this.$router.push("/detail");
      console.log("hello")
    },

    logout() {
      this.$store.dispatch("logout");
    },


    async create() {
      console.log("hello")
      // //textの全データ取得
      const docSnap = await getDocs(collection(db, "text"));
      this.datas = docSnap.docs;

      //image取得 uidとfileNameが必要

      for (let data of this.datas) {
        let fileName = data._document.data.value.mapValue.fields.fileName.stringValue;
        let uid = data._document.data.value.mapValue.fields.userID.stringValue;
        const storage = getStorage();
        await getDownloadURL(ref(storage, uid + "/" + fileName)).then((url) => {
          //リアクティブにするには直接代入はダメ
          this.$set(data, "url", url);
        });
      }

      let i = 0;
      docSnap.forEach((doc) => {
        this.id[i] = doc.id;
        i++;
      });
    },

    async addheart(index) {
      this.good = 0;
      const Ref = doc(db, "text", this.id[index]);
      console.log("index", this.id[index]);
      try {
        this.good =this.datas[index]._document.data.value.mapValue.fields.good.integerValue;
        this.good++;
        await updateDoc(Ref, {good: this.good,});
      } catch (e) {
        alert("Error adding document: ", e);
      }
      this.create();
    },
  },
};
</script>