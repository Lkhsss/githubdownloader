<script setup>
import { ref, watch } from "vue";
import { invoke } from "@tauri-apps/api/core";
import "element-plus/dist/index.css";
import { h } from 'vue'
import { ElNotification } from 'element-plus'
import { tr } from "element-plus/es/locales.mjs";
const origin_data = ref({});
const url = ref("");
const data = ref({});
const ifshow = ref(false);
const ifdata = ref(false);

watch(origin_data, async (newdata, olddata) => {
  if (newdata["success"]) {
    ifdata.value = true;
    data.value = newdata["data"];
  } else {
    ifdata.value = false;
  }
});
watch(url, async (newdata, olddata) => {
  if (newdata.value == '') { ifshow.value = false; } else {
    ifshow.value = true;
  }
});


async function greet() {
  // Learn more about Tauri commands at https://tauri.app/develop/calling-rust/

  ifdata.value = false;
  ElNotification({
    title: '查询数据中',
    duration: 2000
  })
  origin_data.value = await invoke("handle", { url: url.value });
  ifdata.value = true;

  ElNotification({
    title: '查询成功！',
    duration: 2000
  })

}

function copy(data) {
  navigator.clipboard.writeText(data).then(() => {
    ElNotification({
    title: '已复制到剪贴板',
    message: h('i', { style: 'color: teal' }, data),
    duration: 2000
  })
    console.log('已复制到剪贴板！');
  }).catch(err => {
    ElNotification({
    title: '复制失败',
    message: h('i', { style: 'color: red' }, "原因："+err),
    duration: 1000
  });
  });
  return false;
}
</script>

<template>
  <el-input type="text" id="url" v-model="url" placeholder="Enter your url" @change="greet" />
  <template v-if="ifshow">


    <template v-if="ifdata">

      <el-collapse expand-icon-position="left" accordion>
        <template v-for="i in data">
          <el-collapse-item :title="i.name" :name="i.tag_name">
            <a :href="i.html_url"><el-tag type="primary" size="large" effect="dark" round>{{ i.tag_name }}</el-tag></a>
            <el-tag type="primary" size="large" effect="light">{{ i.target_commitish }}</el-tag><br><br>
            <el-button type="primary" size="large" effect="dark" @click="copy(i.tarball_url)"
              title="点击复制">源码链接(tar)</el-button>
            <el-button type="primary" size="large" effect="dark" @click="copy(i.zipball_url)"
              title="点击复制">源码链接(zip)</el-button>
            <el-descriptions title="Releases信息" column=2 border>
              <el-descriptions-item label="Release id">{{ i.id }}</el-descriptions-item>
              <el-descriptions-item label="Author">{{ i.author.login }}</el-descriptions-item>
              <el-descriptions-item label="created time">{{ i.created_at }}</el-descriptions-item>
              <el-descriptions-item label="published time">{{ i.published_at }}</el-descriptions-item>
            </el-descriptions>
            <el-collapse accordion>
              <template v-for="x in i.assets">
                <el-collapse-item :title="x.name">
                  <el-descriptions title="下载" column=1 border>
                    <el-descriptions-item label="Release 名称">{{ x.name }}</el-descriptions-item>
                    <el-descriptions-item label="Release 链接" truncated>
                      <!-- <a href="" @click="copy(x.browser_download_url)" title="点击复制"> {{ x.browser_download_url }}</a>
                  -->
                      <el-button type="primary" size="large" effect="dark" @click="copy(x.browser_download_url)"
                        title="点击复制">{{ x.browser_download_url }}</el-button>
                    </el-descriptions-item>
                    <el-descriptions-item label="创建时间">{{ x.created_at }}</el-descriptions-item>
                    <el-descriptions-item label="更新时间">{{ x.updated_at }}</el-descriptions-item>

                  </el-descriptions>
                </el-collapse-item></template></el-collapse>


          </el-collapse-item>
        </template>
      </el-collapse>
    </template>

      <el-skeleton :rows="10" animated  v-else /> 


  </template>
  <template v-else><el-empty description="暂无数据"></el-empty></template>

</template>
