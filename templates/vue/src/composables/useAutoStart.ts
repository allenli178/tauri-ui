import { disable, enable, isEnabled } from '@tauri-apps/plugin-autostart'

export async function useAutoStart() {
  const enableAutoStart = async () => {
    try {
      await enable()
    }
    catch (error) {
      console.error(error)
    }
  }

  const disableAutoStart = async () => {
    try {
      await disable()
    }
    catch (error) {
      console.error(error)
    }
  }

  const isAutoStartEnabled = async () => {
    try {
      return await isEnabled()
    }
    catch (error) {
      console.error(error)
    }
  }

  return { enableAutoStart, disableAutoStart, isAutoStartEnabled }
}
