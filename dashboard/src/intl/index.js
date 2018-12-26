import antdZhHans from 'antd/lib/locale-provider/zh_CN'
import antdZhHant from 'antd/lib/locale-provider/zh_TW'
import antdEnUS from 'antd/lib/locale-provider/en_US'
import Cookies from 'js-cookie'
import 'moment/locale/zh-cn'
import 'moment/locale/zh-tw'
import {addLocaleData} from 'react-intl'
import dataEn from 'react-intl/locale-data/en'
import dataZh from 'react-intl/locale-data/zh'

import enUS from './en-US'
import zhHant from './zh-Hant'
import zhHans from './zh-Hans'

const KEY = "locale"

export const set = (l) => {
  Cookies.set(KEY, l)
  localStorage.setItem(KEY, l, {
    expires: 1 << 16,
    path: '/'
  })
}

export const get = () => {
  addLocaleData([
    ...dataEn,
    ...dataZh
  ])
  const locale = Cookies.get(KEY) || localStorage.getItem(KEY) || 'en-US'
  switch (locale) {
    case 'zh-Hans':
      return {moment: 'zh-cn', antd: antdZhHans, locale, messages: zhHans}
    case 'zh-Hant':
      return {moment: 'zh-tw', antd: antdZhHant, locale, messages: zhHant}
    default:
      return {moment: 'en', antd: antdEnUS, locale: 'en-US', messages: enUS}
  }
}
