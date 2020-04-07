
import jdk.nashorn.internal.parser.JSONParser;

import java.io.BufferedReader;
import java.io.IOException;
import java.io.InputStreamReader;
import java.net.HttpURLConnection;
import java.net.URL;
import java.net.URLConnection;

public class Test {


    public static void main(String[] args) throws IOException, InterruptedException {

        System.out.println("Test 1: search");
        String s1 = search("graduate school");
        if(s1.contains("Xen and the Art of Surviving Graduate School") && s1.contains("Cooking for the Impatient Graduate Student")){
            System.out.println("2 correct books found");
            System.out.println("Test 1 PASS");
        }else {
            System.out.println("Test 1 FAIL");
        }

        System.out.println("Test 2: search fail");
        String s2 = search("nope");
        if(s2.contains("no book under the topic is found")){
            System.out.println("Test 2 PASS");
        }else {
            System.out.println("Test 2 FAIL");
        }




        System.out.println("Test 3: lookup");
        String s3 = lookup("1");
        if(s3.contains("How to get a good grade in 677 in 20 minutes a day")){
            System.out.println("Test 3 PASS");
        }else {
            System.out.println("Test 3 FAIL");
        }


        System.out.println("Test 4: buy one");
        String s4 = buy("1");
        System.out.println(s4);

        if(s4.contains("9") && s4.contains("How to get a good grade in 677 in 20 minutes a day") ){
            System.out.println("Test 4 PASS");
        }else {
            System.out.println("Test 4 FAIL");
        }
        System.out.println("Test 5: buy them all");
        buy("2");
        buy("2");
        buy("2");
        buy("2");
        buy("2");
        buy("2");
        buy("2");
        buy("2");
        buy("2");
        buy("2");
        String s5 = buy("2");
        System.out.println(s5);

        if(s5.contains("out of stock")){
            System.out.println("Test 5 PASS");
        }else {
            System.out.println("Test 5 FAIL");
        }

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
