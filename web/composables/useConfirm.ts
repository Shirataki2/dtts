import type { NavigationGuardNext } from 'vue-router'

export const useConfirm = (isDirty: Ref<boolean>) => {
  const { showDialog, close } = useDialog<boolean>()
  const confirmBrowserRouter = (e: BeforeUnloadEvent) => {
    if (isDirty.value) {
      e.preventDefault()
      e.returnValue = '内容を破棄しますか？'
    }
  }

  const confirmVueRouter = async (next: NavigationGuardNext) => {
    console.log('confirm')
    if (isDirty.value) {
      const res = await showDialog({
        title: '内容を破棄しますか？',
        description: '現在の入力内容を破棄して移動しますか？',
        actions: [
          {
            text: 'はい',
            color: 'error',
            handler: () => {
              close()
              return true
            },
          },
          {
            text: 'いいえ',
            color: 'primary',
            handler: () => {
              close()
              return false
            },
          },
        ],
      })
      if (res.value) {
        window.removeEventListener('beforeunload', confirmBrowserRouter)
      }
      next(!!res.value)
    } else {
      next()
    }
  }
  onBeforeRouteLeave(async (_to, _from, next) => {
    await confirmVueRouter(next)
  })

  onBeforeRouteUpdate(async (_to, _from, next) => {
    await confirmVueRouter(next)
  })
  if (process.client) {
    window.addEventListener('beforeunload', confirmBrowserRouter)
  }
}
