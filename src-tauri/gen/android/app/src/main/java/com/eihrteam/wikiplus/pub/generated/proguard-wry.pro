# THIS FILE IS AUTO-GENERATED. DO NOT MODIFY!!

# Copyright 2020-2023 Tauri Programme within The Commons Conservancy
# SPDX-License-Identifier: Apache-2.0
# SPDX-License-Identifier: MIT

-keep class com.eihrteam.wikiplus.pub.* {
  native <methods>;
}

-keep class com.eihrteam.wikiplus.pub.WryActivity {
  public <init>(...);

  void setWebView(com.eihrteam.wikiplus.pub.RustWebView);
  java.lang.Class getAppClass(...);
  java.lang.String getVersion();
}

-keep class com.eihrteam.wikiplus.pub.Ipc {
  public <init>(...);

  @android.webkit.JavascriptInterface public <methods>;
}

-keep class com.eihrteam.wikiplus.pub.RustWebView {
  public <init>(...);

  void loadUrlMainThread(...);
  void loadHTMLMainThread(...);
  void evalScript(...);
}

-keep class com.eihrteam.wikiplus.pub.RustWebChromeClient,com.eihrteam.wikiplus.pub.RustWebViewClient {
  public <init>(...);
}
