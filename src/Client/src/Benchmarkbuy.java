import java.io.BufferedReader;
import java.io.IOException;
import java.io.InputStreamReader;
import java.net.HttpURLConnection;
import java.net.URL;
import java.net.URLConnection;

public class Benchmarkbuy {


    public static void main(String[] args) throws IOException, InterruptedException {

        long startTime = System.currentTimeMillis();

        for(int i =0;i<1000;i++){
        buy("1");
            }

        long endTime = System.currentTimeMillis();

        System.out.println("That took " + (endTime - startTime) + " milliseconds");


    }

    private static String buy(String lastline) throws IOException {
        URL request3 = new URL("http://128.119.243.147:34841/buy/"+lastline);
        URLConnection yc3 = request3.openConnection();

        HttpURLConnection http = (HttpURLConnection)yc3;
        http.setRequestMethod("PUT"); // PUT is another valid option
        http.setDoOutput(true);

        BufferedReader r3 = new BufferedReader(new InputStreamReader(
                yc3.getInputStream()));
        StringBuilder sb = new StringBuilder();

        String inputLine3;

        while ((inputLine3 = r3.readLine()) != null)
            sb.append(inputLine3);
        r3.close();
        return sb.toString();
    }

    private static String lookup(String lastline) throws IOException {

        URL request2 = new URL("http://128.119.243.147:34841/lookup/"+lastline);
        URLConnection yc2 = request2.openConnection();
        BufferedReader r2 = new BufferedReader(new InputStreamReader(
                yc2.getInputStream()));
        StringBuilder sb = new StringBuilder();

        String inputLine2;

        while ((inputLine2 = r2.readLine()) != null)
            sb.append(inputLine2);
        r2.close();
        return sb.toString();

    }

    private static String search(String lastline) throws IOException {
        lastline = lastline.replace(" ","-");

        URL request1 = new URL("http://128.119.243.147:34841/search/"+lastline);
        URLConnection yc = request1.openConnection();
        BufferedReader r1 = new BufferedReader(new InputStreamReader(
                yc.getInputStream()));

        String inputLine;
        StringBuilder sb = new StringBuilder();

        while ((inputLine = r1.readLine()) != null)
            sb.append(inputLine);
        r1.close();
        return sb.toString();

    }
}
