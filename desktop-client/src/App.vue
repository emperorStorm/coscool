<template>
  <a-config-provider :theme="themeConfig">
    <router-view />
  </a-config-provider>
</template>

<script setup lang="ts">
const themeConfig = {
  token: {
    colorPrimary: '#0f8f83',
    colorSuccess: '#49a942',
    colorWarning: '#f08a24',
    colorError: '#d9363e',
    borderRadius: 6,
    fontSize: 14,
    controlHeight: 34,
    colorText: '#273244',
    colorTextSecondary: '#607086',
    colorBorder: '#e1e7ef'
  },
  components: {
    Layout: {
      headerBg: '#ffffff',
      siderBg: '#f3f6f8'
    },
    Table: {
      headerBg: '#f3f7f9',
      headerColor: '#314052',
      rowHoverBg: '#f7fbfb'
    },
    Menu: {
      itemSelectedBg: '#e3f6f3',
      itemSelectedColor: '#0f8f83'
    }
  }
}
</script>
