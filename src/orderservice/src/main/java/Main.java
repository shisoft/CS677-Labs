import org.json.simple.JSONObject;
import org.slf4j.Logger;
import org.slf4j.LoggerFactory;

import java.io.BufferedReader;
import java.io.FileReader;
import java.io.IOException;
import java.io.InputStreamReader;
import java.net.HttpURLConnection;
import java.net.URL;
import java.net.URLConnection;

import static spark.Spark.*;

public class Main {

    private static Logger logger = LoggerFactory.getLogger(Main.class);

    public static void main(String[] args) {

        port(34842);
//todo log
        //todo At the beginning of each run, the Catalog log should
        // include one or more entries recording the initial state of
        // the catalog database (e.g. how many books were intially inserted)

        post("/buy/:topic", (req,res)->{
            logger.info("look " +req.params(":topic"));
            return "you bought, "+ query(req.params(":topic"));
        });


    }
    public static String query(String topic) throws IOException {

        URL request2 = new URL("http://0.0.0.0:34843/buy/"+topic);
        logger.info("http.getContent().toString() ");

        URLConnection yc2 = request2.openConnection();
        HttpURLConnection http = (HttpURLConnection)yc2;
        http.setRequestMethod("POST"); // PUT is another valid option
        http.setDoOutput(true);

        BufferedReader r = new BufferedReader(new InputStreamReader(
                yc2.getInputStream()));

        String inputLine3;
        StringBuilder sb = new StringBuilder();

        while ((inputLine3 = r.readLine()) != null)
            sb.append(inputLine3);

        r.close();

        logger.info("http.getContent().toString() "+sb.toString() );

        return sb.toString();

    }


}