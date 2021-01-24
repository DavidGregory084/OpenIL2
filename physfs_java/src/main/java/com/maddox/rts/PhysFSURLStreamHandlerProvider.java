package com.maddox.rts;

import java.io.IOException;
import java.net.URL;
import java.net.URLConnection;
import java.net.URLStreamHandler;
import java.net.spi.URLStreamHandlerProvider;

public class PhysFSURLStreamHandlerProvider extends URLStreamHandlerProvider {
    @Override
    public URLStreamHandler createURLStreamHandler(String protocol) {
        if ("physfs".equals(protocol)) {
            return new URLStreamHandler() {
                @Override
                protected URLConnection openConnection(URL url) throws IOException {
                    return new PhysFSURLConnection(url);
                }
            };
        }
        return null;
    }
}
