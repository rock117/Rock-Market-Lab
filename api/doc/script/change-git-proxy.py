import winreg
import os

def get_proxy_settings():
    proxy_settings = {}
    try:
        # 打开注册表键
        key = winreg.OpenKey(winreg.HKEY_CURRENT_USER, r"Software\Microsoft\Windows\CurrentVersion\Internet Settings")
        
        # 获取代理启用状态
        proxy_enabled, _ = winreg.QueryValueEx(key, "ProxyEnable")
        proxy_settings["ProxyEnable"] = proxy_enabled

        if proxy_enabled:
            # 获取代理服务器地址
            proxy_server, _ = winreg.QueryValueEx(key, "ProxyServer")
            proxy_settings["ProxyServer"] = proxy_server

            # 获取代理跳过的地址（可选）
            try:
                proxy_bypass, _ = winreg.QueryValueEx(key, "ProxyOverride")
                proxy_settings["ProxyOverride"] = proxy_bypass
            except FileNotFoundError:
                proxy_settings["ProxyOverride"] = None

    except Exception as e:
        proxy_settings["error"] = str(e)

    return proxy_settings


proxy_info = get_proxy_settings()
proxyServer = proxy_info["ProxyServer"]
print(f"proxyServer: {proxyServer}")
os.system("git config --global http.proxy http://" + proxyServer)

