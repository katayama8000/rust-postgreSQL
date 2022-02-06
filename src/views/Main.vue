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
      <!-- <v-btn @click="listen">listen</v-btn> -->
    </v-row>
    <br />
    <hr />
    <br />
    <v-row justify="center" class="text-center">
      <v-col cols="3" v-for="(data, index) in datas" :key="index">
        <v-card>
          <v-card-title class="headline"
            >{{ data.data().textName }}({{ data.data().uniName }})</v-card-title
          >
          <v-card-subtitle>{{ data.data().major }}</v-card-subtitle>
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
            {{ data.data().good }}
          </v-card-actions>
        </v-card>
        {{data.add}}
      </v-col>
    </v-row>
    {{ good }}
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
import { onSnapshot } from "firebase/firestore";
import { query } from "firebase/firestore";
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
      docId: [],
    };
  },

  async created() {
    this.create();
    //  onSnapshot(doc(db, "text", "AnrpPw5zb7yPlpWZocgF"), (doc) => {
    //     const source = doc.metadata.hasPendingWrites ? "Local" : "Server";
    //     console.log(source, " data: ", doc.data().good);
    //     //this.good = doc.data().good
    //   });
  },

  mounted() {
    //指定されたところ一つ
    onSnapshot(doc(db, "text", "AnrpPw5zb7yPlpWZocgF"), (doc) => {
      const source = doc.metadata.hasPendingWrites ? "Local" : "Server";
      console.log(source, " data: ", doc.data().good);
      this.good = doc.data().good;
      //this.$set(this.good, doc.data().good);
    });

    //textすべてを見れる
    const q = query(collection(db, "text"));
    onSnapshot(q, (querySnapshot) => {
    //const cities = [];
    let i = 0
    querySnapshot.forEach((doc) => {
      this.$set(this.datas[i], "add" , doc.data().good);
       i++
    });
       console.log("hello:", this.datas);
    });
  },

  computed: {},

  watch: {
    good: function () {},
  },

  methods: {
    toDetailPage(index) {
      this.$store.commit("upDataDetailText", this.datas[index].id);
      this.$router.push("/detail");
    },

    logout() {
      this.$store.dispatch("logout");
    },

    async create() {
      // //textの全データ取得
      const docSnap = await getDocs(collection(db, "text"));
      this.datas = docSnap.docs;

      //image取得 uidとfileNameが必要

      for (let data of this.datas) {
        console.log(data.data().fileName);
        let fileName = data.data().fileName;
        let uid = data.data().userID;
        const storage = getStorage();
        await getDownloadURL(ref(storage, uid + "/" + fileName)).then((url) => {
          //リアクティブにするには直接代入はダメ
          this.$set(data, "url", url);
        });
        this.$set(data, "heart", this.good);
      }

      let i = 0;
      docSnap.forEach((doc) => {
        this.id[i] = doc.id;
        i++;
      });
    },

    async addheart(index) {
      //this.good = 0;
      const Ref = doc(db, "text", this.id[index]);
      console.log("index", this.id[index]);
      try {
        //this.good = this.datas[index].data().good;
        this.good++;
        // await updateDoc(Ref, { good: this.good });
        // this.$set(this.datas[index], "add" , this.good);
        await updateDoc(Ref, { add: this.good });
        //this.$set(this.datas[index], "add" , this.good);
      } catch (e) {
        alert("Error adding document: ", e);
      }

      // onSnapshot(doc(db, "text", "AnrpPw5zb7yPlpWZocgF"), (doc) => {
      //   const source = doc.metadata.hasPendingWrites ? "Local" : "Server";
      //   console.log(source, " data: ", doc.data().good);
      //   this.good = doc.data().good;
      // });

      console.log(this.good);
      //this.create();
    },

    // async listen() {
    //   onSnapshot(doc(db, "text", "AnrpPw5zb7yPlpWZocgF"), (doc) => {
    //     const source = doc.metadata.hasPendingWrites ? "Local" : "Server";
    //     console.log(source, " data: ", doc.data());
    //   });
    // },
  },
};
</script>