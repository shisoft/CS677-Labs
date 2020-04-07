import org.json.simple.JSONArray;
import org.json.simple.JSONObject;
import org.json.simple.parser.JSONParser;
import org.json.simple.parser.ParseException;
//import org.slf4j.Logger;
import org.slf4j.LoggerFactory;
import sun.rmi.runtime.Log;

import java.nio.channels.FileChannel;
import java.nio.channels.FileLock;
import java.nio.channels.OverlappingFileLockException;
import java.util.concurrent.atomic.AtomicLong;
import java.util.logging.FileHandler;
import java.util.logging.Logger;

import java.io.*;
import java.util.logging.SimpleFormatter;

import static java.lang.Thread.sleep;
import static spark.Spark.*;

public class Main {
    private final static Logger logger = Logger.getLogger(Main.class.getName());
    private static FileHandler fh;
    //private static Logger logger = LoggerFactory.getLogger(Main.class);

    public static void main(String[] args) throws IOException {

        fh = new FileHandler("CATALOG.log");
        logger.addHandler(fh);
        SimpleFormatter formatter = new SimpleFormatter();
        fh.setFormatter(formatter);

        // the following statement is used to log any messages
        AtomicLong ct = new AtomicLong();

        logger.info("Catalog running ");
        setuplog();
        port(34842);
//todo log
        //todo At the beginning of each run, the Catalog log should
        // include one or more entries recording the initial state of
        // the catalog database (e.g. how many books were intially inserted)
        get("/search/:topic", (req,res)->{
            long startTime = System.currentTimeMillis();
            logger.info("search for topic");
            logger.info("topic"+req.params(":topic"));
            JSONObject re = search(req.params(":topic"));
            long endTime = System.currentTimeMillis();
            ct.addAndGet((endTime - startTime));
            logger.info(ct+"tim!!!!se");
            return re;

        });
        get("/lookup/:id", (req,res)->{
            long startTime = System.currentTimeMillis();
            JSONObject re = lookup(req.params(":id"));
            long endTime = System.currentTimeMillis();
            ct.addAndGet((endTime - startTime));
            logger.info(ct+"tim!!!!se");
            return re;

        });
        put("/buy/:id", (req,res)->{
            long startTime = System.currentTimeMillis();
            String re = buy(req.params(":id"));
            long endTime = System.currentTimeMillis();
            ct.addAndGet((endTime - startTime));
            logger.info(ct+"tim!!!!buy");
            return re;

        });
        get("/query/:id", (req,res)->{
            long startTime = System.currentTimeMillis();
            String re = querybuy(req.params(":id"));
            long endTime = System.currentTimeMillis();
            ct.addAndGet((endTime - startTime));
            logger.info(ct+"tim!!!!query");
            return re;

        });



    }

    private static void setuplog() throws IOException {
        logger.info("======================================= ");

        BufferedReader csvReader = new BufferedReader(new FileReader("inventory.csv"));
        String row;
        boolean contains = false;
        JSONObject ob = new JSONObject();
        while ((row = csvReader.readLine()) != null) {
            String[] data = row.split(",");
            logger.info("book id "+data[1]+" has "+data[2]+" in stock");
        }

        csvReader.close();
        logger.info("======================================= ");

    }

    private static String  querybuy(String id) throws IOException {
        JSONObject info = lookup(id);
        if (info.containsKey("message")){
            return "false";
        }
        logger.info("stock for "+id+" is "+info.get("stock").toString());
        if (Integer.parseInt(info.get("stock").toString())>0){
            return "true";
        }
        return "false";

    }

    private static String   buy(String id) throws IOException, InterruptedException {
        logger.info("Buying "+id);

        RandomAccessFile file = new RandomAccessFile("inventory.csv", "rw");
        FileChannel channel = file.getChannel();
        FileLock lock = channel.tryLock();
        while(lock==null){
            sleep(1000);
            lock = channel.tryLock();
        }

        BufferedReader csvReader = new BufferedReader(new FileReader("inventory.csv"));

        String row;
        boolean contains = false;
        String text ="";
        String re = "";
        while ((row = csvReader.readLine()) != null) {
            String[] data = row.split(",");
            if(data[1].equals(id)){
                contains=true;
                text+= data[0]+","+data[1]+","+(Integer.valueOf(data[2])-1)+","+data[3]+","+data[4]+","+"\n";
                re = data[0] +"---"+(Integer.valueOf(data[2])-1);
            }else {
                text+=row+"\n";
            }

        }
        csvReader.close();

        FileWriter writer = new FileWriter("inventory.csv");
        writer.write(text);
        logger.info("Writting "+id+" after buy");
        writer.close();

        lock.release();

        file.close();
        channel.close();

        if (!contains){
            logger.info("doesnt not have "+id);
            re="message"+"no book under the topic is found";
        }
        return re;

    }

    public static JSONObject search(String topic) throws IOException {
        logger.info("Searching for "+topic);


        topic = topic.replace("-"," ");

        BufferedReader csvReader = new BufferedReader(new FileReader("inventory.csv"));
        String row;
        boolean contains = false;
        JSONObject ob = new JSONObject();
        while ((row = csvReader.readLine()) != null) {
            String[] data = row.split(",");
            if(data[4].equals(topic)){
                contains=true;
                ob.put(data[0],Integer.valueOf(data[1]));
            }
        }

        csvReader.close();
        if (!contains){
            ob.put("message","no book under the topic is found");
        }
        return ob;
    }


    public static JSONObject lookup(String id) {
        //todo sql

        BufferedReader csvReader = null;
        boolean contains = false;
        JSONObject ob1 = new JSONObject();

        try {
            csvReader = new BufferedReader(new FileReader("inventory.csv"));

        logger.info("look up for id"+ id);
        String row;
        while ((row = csvReader.readLine()) != null) {
            String[] data = row.split(",");
            if(data[1].equals(id)){
                contains=true;
                ob1.put("name",data[0]);
                ob1.put("id",Integer.valueOf(data[1]));
                ob1.put("stock",Integer.valueOf(data[2]));
                ob1.put("price",data[3]);
                ob1.put("topic",data[4]);
                break;
            }
        }
            csvReader.close();
        } catch (FileNotFoundException e) {
            e.printStackTrace();
        } catch (IOException e) {
            e.printStackTrace();
        }

        if (!contains){
            ob1.put("message","no book under the id is found");
        }
        return ob1;

    }

}