<template>
  <v-app style="height: 100%;">
    <v-container style="height: 100%;">
      <v-card style="height:20%; padding: 5px;">
        我的UID:<span style="font-size: 20px; font-weight: bolder;">{{ uid }}</span>
        <form @submit.prevent="connect" v-if="!established">
          <v-row>
            <v-col cols="2">
              <span>对方的UID:</span>
            </v-col>
            <v-col cols="8">
              <v-text-field v-model="target_uid" />
            </v-col>
            <v-col cols="2">
              <v-btn type="submit" v-if="!established">连接</v-btn>
            </v-col>
          </v-row>
        </form>

      </v-card>
      <div ref="messageBox" style="height: 280px; overflow:auto; padding: 10px;">
        <v-timeline side="end">
          <v-timeline-item v-for="item in messageList" :dot-color="item.color" size="small">
            {{ item.message }}
          </v-timeline-item>
        </v-timeline>
      </div>
      <v-divider style="margin: 5px;"></v-divider>
      <div style="height: 20%;">
        <form @submit.prevent="send">
          <v-row style="height: 120px;">
            <v-col cols="10">
              <v-textarea rows="3" row-height="15" label="消息" v-model="message" variant="solo-filled"></v-textarea>
            </v-col>
            <v-col cols="2">
              <v-btn type="submit" color="indigo-darken-3">发送</v-btn>
            </v-col>
          </v-row>

        </form>
          <v-row>
            <v-col cols="10">
              <v-progress-linear
                v-model="fileProgress"
                color="green"
                style="margin-top: 10px;"
              ></v-progress-linear>
            </v-col>
            <v-col cols="2">
              <v-btn color="indigo-darken-3" @click="sendFile">发送文件</v-btn>
            </v-col>
          </v-row>
      </div>
    </v-container>
  </v-app>
</template>

<script setup lang="ts">
import { invoke } from '@tauri-apps/api';
import { open } from '@tauri-apps/api/dialog';
import { listen } from '@tauri-apps/api/event';
import { Ref, onMounted, ref } from 'vue';

const uid = ref('');
const target_uid = ref('');
const established_uid = ref('');
const established = ref(false);

const message = ref('');
const fileProgress = ref(0);

const messageList = ref([]) as Ref<any[]>;
const messageBox = ref();

onMounted(async () => {
  invoke('my_uid_get').then((my_uid: any) => {
    uid.value = my_uid;
  })
  invoke('init_process');
  fetchMessages();
})

const connect = async () => {
  invoke('connect', { targetUid: target_uid.value });
  listenConnected();
}

let listenConnectedOnce = true;

const listenConnected = async () => {
  if (!listenConnectedOnce) {
    return;
  }
  listenConnectedOnce = false;

  return await listen('established_connect', (event: any) => {
      established_uid.value = event.payload.target_uid;
      established.value = event.payload.target_uid.length > 0;
  });
}

const send = async () => {
  if (message.value.length == 0) {
    return;
  }
  invoke('send_message', { message: message.value });
  message.value = '';
}

const sendFile = async () => {
  let filePath = await open();
  invoke('send_file', {filePath: filePath});
}

const fetchMessages = async () => {
  setInterval(() => {
      invoke('fetch_messages').then((messagesJson: any) => {
        messageList.value.length = 0;
        const messages = JSON.parse(messagesJson);
        for (const m of messages) {
          messageList.value.push({color: m.from == 0 ? 'green' : 'pink', message: m.content, timestamp: m.timestamp});
        }
        messageList.value.sort((m1: any, m2: any) => m1.timestamp - m2.timestamp);
        if (messageBox.value != null) {
          messageBox.value.scrollTop = messageBox.value.scrollHeight??0;
        }
      })
    }, 1000);
  }
</script>

<style scoped>
</style>
