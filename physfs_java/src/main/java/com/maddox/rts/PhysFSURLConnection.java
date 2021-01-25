package com.maddox.rts;

import java.io.IOException;
import java.io.InputStream;
import java.net.URL;
import java.net.URLConnection;

public class PhysFSURLConnection extends URLConnection {
    private PhysFSInputStream inputStream;
    private boolean connected = false;

    public PhysFSURLConnection(URL url) {
        super(url);
    }

    @Override
    public int getContentLength() {
        if (!connected) {
            return -1;
        }

        long len = inputStream.fileLength();
        if (len > Integer.MAX_VALUE)
            return -1;
        else
            return (int) len;
    }

    @Override
    public InputStream getInputStream() throws IOException {
        if (!connected) {
            connect();
        }

        return inputStream;
    }

    @Override
    public void connect() throws IOException {
        try {
            inputStream = new PhysFSInputStream(getURL().getPath());
        } catch (PhysFSException exc) {
            throw new IOException(exc);
        }
    }
}
