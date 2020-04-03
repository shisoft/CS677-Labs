import javax.swing.plaf.basic.BasicPasswordFieldUI;
import java.io.IOException;
import java.net.http.HttpClient;
import java.net.http.HttpRequest;
import java.net.URI;
import java.net.http.HttpResponse;
import java.util.Scanner;

public class Client {


    public static void main(String[] args) throws IOException, InterruptedException {
        Scanner in = new Scanner(System.in);
        String lastline = "null";
        HttpClient client = HttpClient.newHttpClient();
        while(!lastline.toLowerCase().equals("quit")){
            System.out.println("Welcome, what would you like to do? search, lookup, buy？type in or indicate by number 1,2,3 or quit");
            switch (in.nextLine().toLowerCase()){
                case "1":
                case "search":
                    System.out.println("What is the topic?");
                    lastline = in.nextLine().toLowerCase();
                    lastline = lastline.replace(" ","-");

                    HttpRequest request = HttpRequest.newBuilder()
                            .uri(URI.create("http://0.0.0.0:4567/search/"+lastline))
                            .build();

                    HttpResponse<String> response = client.send(request,
                            HttpResponse.BodyHandlers.ofString());

                    System.out.println(response.body());
                    break;

                case "2":
                case "lookup":
                    System.out.println("What is the item number?");
                    lastline = in.nextLine().toLowerCase();
                    HttpRequest requestlook = HttpRequest.newBuilder()
                            .uri(URI.create("http://0.0.0.0:4567/lookup/"+lastline))
                            .build();

                    HttpResponse<String> responselook = client.send(requestlook,
                            HttpResponse.BodyHandlers.ofString());

                    System.out.println(responselook.body());
                    break;
                case "3":
                case "buy":
                    System.out.println("What is the item number?");
                    lastline = in.nextLine().toLowerCase();
                    HttpRequest requestbuy = HttpRequest.newBuilder()
                            .uri(URI.create("http://0.0.0.0:4567/buy/"+lastline))
                            .build();

                    HttpResponse<String> responsebuy = client.send(requestbuy,
                            HttpResponse.BodyHandlers.ofString());

                    System.out.println(responsebuy.body());
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
