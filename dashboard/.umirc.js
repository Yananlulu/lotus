export default {
  base: '/my',
  plugins: [
    [
      'umi-plugin-react', {
        dva: {
          immer: true
        },
        antd: true,
        dynamicImport: {
          webpackChunkName: true,
          loadingComponent: 'components/Loading'
        },
        locale: {
          default: 'zh-CN',
          baseNavigator: true,
          antd: true
        }
      }
    ]
  ]
};
