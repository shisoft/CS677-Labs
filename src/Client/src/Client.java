
import javax.swing.plaf.basic.BasicPasswordFieldUI;
import java.io.BufferedReader;
import java.io.IOException;
import java.io.InputStreamReader;
import java.net.*;

import java.util.Scanner;

public class Client {


    public static void main(String[] args) throws IOException, InterruptedException {
        Scanner in = new Scanner(System.in);
        String lastline = "null";

        while(!lastline.toLowerCase().equals("quit")){
            System.out.println("Welcome, what would you like to do? search, lookup, buy？type in or indicate by number 1,2,3 or quit");
            switch (in.nextLine().toLowerCase()){
                case "1":
                case "search":
                    System.out.println("What is the topic?");
                    lastline = in.nextLine().toLowerCase();
                    lastline = lastline.replace(" ","-");

                    URL request1 = new URL("http://128.119.243.147:34841/search/"+lastline);
                    URLConnection yc = request1.openConnection();
                    BufferedReader r1 = new BufferedReader(new InputStreamReader(
                        yc.getInputStream()));

                    String inputLine;

                    while ((inputLine = r1.readLine()) != null)
                        System.out.println(inputLine);
                    r1.close();
                    break;

                case "2":
                case "lookup":
                    System.out.println("What is the item number?");
                    lastline = in.nextLine().toLowerCase();

                    URL request2 = new URL("http://128.119.243.147:34841/lookup/"+lastline);
                    URLConnection yc2 = request2.openConnection();
                    BufferedReader r2 = new BufferedReader(new InputStreamReader(
                            yc2.getInputStream()));

                    String inputLine2;

                    while ((inputLine2 = r2.readLine()) != null)
                        System.out.println(inputLine2);
                    r2.close();

                    break;
                case "3":
                case "buy":
                    System.out.println("What is the item number?");
                    lastline = in.nextLine().toLowerCase();


                    URL request3 = new URL("http://128.119.243.147:34841/buy/"+lastline);
                    URLConnection yc3 = request3.openConnection();

                    HttpURLConnection http = (HttpURLConnection)yc3;
                    http.setRequestMethod("PUT"); // PUT is another valid option
                    http.setDoOutput(true);

                    BufferedReader r3 = new BufferedReader(new InputStreamReader(
                            yc3.getInputStream()));

                    String inputLine3;

                    while ((inputLine3 = r3.readLine()) != null)
                        System.out.println(inputLine3);
                    r3.close();
                    break;
                case "quit":
                    lastline = "quit";
                    break;
                default:
                    System.out.println("sorry I don't understand");
                    break;

            }
        }


    }
}
