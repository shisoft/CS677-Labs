import java.io.BufferedReader;
import java.io.IOException;
import java.io.InputStreamReader;
import java.net.HttpURLConnection;
import java.net.URL;
import java.net.URLConnection;
import java.util.logging.FileHandler;
import java.util.logging.Logger;
import java.util.logging.SimpleFormatter;

import static spark.Spark.port;
import static spark.Spark.put;

public class Main {

    private final static java.util.logging.Logger logger = Logger.getLogger(Main.class.getName());
    private static FileHandler fh;

    public static void main(String[] args) throws IOException {
//log set up
        fh = new FileHandler("ORDER.log");
        logger.addHandler(fh);
        SimpleFormatter formatter = new SimpleFormatter();
        fh.setFormatter(formatter);
        logger.info("Order running ");

        port(34843);

        put("/buy/:topic", (req,res)->{
            logger.info("look " +req.params(":topic"));
            return  query(req.params(":topic"));
        });


    }
    public static String query(String topic) throws IOException {
//when a buy is called from front end, it sends a query to catalog to check if a book is available,
        URL request2 = new URL("http://128.119.243.164:34842/query/"+topic);
        logger.info("making query request");

        URLConnection yc2 = request2.openConnection();

        BufferedReader r = new BufferedReader(new InputStreamReader(
                yc2.getInputStream()));

        String inputLine3;
        StringBuilder sb = new StringBuilder();

        while ((inputLine3 = r.readLine()) != null)
            sb.append(inputLine3);

        r.close();
        if (sb.toString().equals("true")){
            //if it is avaliable, then it sends the put request to the buy API call to purchase.
            logger.info("making buying request");

            URL request3 = new URL("http://128.119.243.164:34842/buy/"+topic);
            URLConnection yc3 = request3.openConnection();
            HttpURLConnection http = (HttpURLConnection)yc3;
            http.setRequestMethod("PUT");
            http.setDoOutput(true);

            BufferedReader r1 = new BufferedReader(new InputStreamReader(
                    http.getInputStream()));

            String inputLine;
            StringBuilder sb1 = new StringBuilder();

            while ((inputLine = r1.readLine()) != null)
                sb1.append(inputLine);
            r1.close();
            logger.info("Successfully bought"+sb1.toString());

            return sb1.toString();
        }//or not it returns out of stock.
        logger.info("run out of stock "+sb.toString() );

        return "out of stock";

    }


}