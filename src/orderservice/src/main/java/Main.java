import org.json.simple.JSONObject;
import org.slf4j.LoggerFactory;

import java.io.BufferedReader;
import java.io.FileReader;
import java.io.IOException;
import java.io.InputStreamReader;
import java.net.HttpURLConnection;
import java.net.URL;
import java.net.URLConnection;
import java.util.logging.FileHandler;
import java.util.logging.Logger;
import java.util.logging.SimpleFormatter;

import static spark.Spark.*;

public class Main {

    private final static java.util.logging.Logger logger = Logger.getLogger(Main.class.getName());
    private static FileHandler fh;
    //private static Logger logger = LoggerFactory.getLogger(Main.class);

    public static void main(String[] args) throws IOException {

        fh = new FileHandler("ORDER.log");
        logger.addHandler(fh);
        SimpleFormatter formatter = new SimpleFormatter();
        fh.setFormatter(formatter);

        port(34843);
//todo log
        //todo At the beginning of each run, the Catalog log should
        // include one or more entries recording the initial state of
        // the catalog database (e.g. how many books were intially inserted)

        put("/buy/:topic", (req,res)->{
            logger.info("look " +req.params(":topic"));
            return  query(req.params(":topic"));
        });


    }
    public static String query(String topic) throws IOException {

        URL request2 = new URL("http://128.119.243.164:34842/query/"+topic);
        logger.info("http.getContent().toString() ");

        URLConnection yc2 = request2.openConnection();

        BufferedReader r = new BufferedReader(new InputStreamReader(
                yc2.getInputStream()));

        String inputLine3;
        StringBuilder sb = new StringBuilder();

        while ((inputLine3 = r.readLine()) != null)
            sb.append(inputLine3);

        r.close();
        if (sb.toString().equals("true")){
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
            return "you bought"+sb1.toString();
        }
        logger.info("run out of stock "+sb.toString() );

        return "out of stock";

    }


}