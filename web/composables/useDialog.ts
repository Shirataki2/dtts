import Emitter from 'eventemitter3'

export const useDialog = <T>() => {
  type DialogOptions = {
    title: string
    description: string
    width?: string
    actions?: {
      text: string
      color?: string
      handler: () => T
    }[]
  }
  const bus = new Emitter()
  const dialog = useState<DialogOptions | null>('dialog', () => null)
  const result = useState<T | null>('dialogResult', () => null)
  const close = () => {
    bus.emit('close')
    dialog.value = null
  }
  const showDialog = async (options: DialogOptions | null) => {
    dialog.value = options
    await new Promise<void>(resolve => {
      bus.once('close', resolve)
    })
    return result
  }
  return {
    dialog,
    result,
    showDialog,
    close,
  }
}
